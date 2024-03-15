pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));
}

pub mod rpc {
    use crate::messages::zmk::{response::Type, Notification, Request, RequestResponse, Response};
    use futures::{channel::mpsc::SendError, lock::Mutex, Future, SinkExt, StreamExt};

    pub mod framing {
        use futures::stream::unfold;
        use futures::stream::StreamExt;
        use futures::Stream;

        fn is_escaped_byte(b: u8) -> bool {
            match b {
                FRAME_SOF | FRAME_ESC | FRAME_EOF => true,
                _ => false,
            }
        }

        pub fn framed(payload: Vec<u8>) -> Vec<u8> {
            let escaped = payload.into_iter().flat_map(|b| {
                if is_escaped_byte(b) {
                    vec![FRAME_ESC, b].into_iter()
                } else {
                    vec![b].into_iter()
                }
            });

            return std::iter::once(FRAME_SOF)
                .chain(escaped)
                .chain(std::iter::once(FRAME_EOF))
                .collect();
        }

        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum FrameParsingErr {
            UnexpectedSof,
            DataBeforeSof,
        }

        #[derive(PartialEq, Clone, Copy, Debug)]
        enum FramingParseState {
            FramingStateIdle,
            FramingStateAwaitingData,
            FramingStateEscaped,
        }

        const FRAME_SOF: u8 = 0xAB;
        const FRAME_ESC: u8 = 0xAC;
        const FRAME_EOF: u8 = 0xAD;

        struct FrameFoldState<T>
        where
            T: Stream<Item = u8>,
        {
            stream: T,
            parse_state: Result<FramingParseState, FrameParsingErr>,
            pending_frame: Vec<u8>,
        }

        async fn get_next_frame<T>(
            mut state: FrameFoldState<T>,
        ) -> Option<(Result<Vec<u8>, FrameParsingErr>, FrameFoldState<T>)>
        where
            T: Stream<Item = u8> + Unpin,
        {
            while let Some(b) = state.stream.next().await {
                match state.parse_state {
                    Ok(FramingParseState::FramingStateIdle) => {
                        // TODO: Stop if we already got a frame
                        match b {
                            FRAME_SOF => {
                                state.parse_state = Ok(FramingParseState::FramingStateAwaitingData)
                            }
                            _ => state.parse_state = Err(FrameParsingErr::DataBeforeSof),
                        };
                    }
                    Ok(FramingParseState::FramingStateAwaitingData) => {
                        state.parse_state = match b {
                            FRAME_SOF => Err(FrameParsingErr::UnexpectedSof),
                            FRAME_ESC => Ok(FramingParseState::FramingStateEscaped),
                            FRAME_EOF => Ok(FramingParseState::FramingStateIdle),
                            data_byte => {
                                state.pending_frame.push(data_byte);

                                Ok(FramingParseState::FramingStateAwaitingData)
                            }
                        };
                    }
                    Ok(FramingParseState::FramingStateEscaped) => {
                        state.pending_frame.push(b);
                    }
                    Err(_) => {
                        state.parse_state = match b {
                            FRAME_SOF => Ok(FramingParseState::FramingStateAwaitingData),
                            _ => Err(FrameParsingErr::DataBeforeSof),
                        }
                    }
                };

                if state.parse_state.is_err() {
                    state.pending_frame.clear();
                    return Some((Err(state.parse_state.clone().err().unwrap()), state));
                } else if state.parse_state == Ok(FramingParseState::FramingStateIdle) {
                    let frame = state.pending_frame.clone();
                    state.pending_frame.clear();

                    return Some((Ok(frame), state));
                }
            }

            None
        }

        pub fn to_frames<T>(stream: T) -> impl Stream<Item = Result<Vec<u8>, FrameParsingErr>>
        where
            T: Stream<Item = u8> + Unpin,
        {
            unfold(
                FrameFoldState {
                    stream,
                    parse_state: Ok(FramingParseState::FramingStateIdle),
                    pending_frame: Vec::with_capacity(64),
                },
                get_next_frame,
            )
        }

        #[test]
        fn test_single_frame() {
            let test = async {
                let stream = futures::stream::iter(vec![FRAME_SOF, 1, 2, 3, FRAME_EOF]);

                let frames_stream = to_frames(stream);

                pin_mut!(frames_stream);

                let parsed = frames_stream
                    .next()
                    .await
                    .expect("Stream had an item")
                    .expect("bytes parsed without parsing error");

                assert_eq!(parsed, vec![1, 2, 3]);
            };

            futures::executor::block_on(test)
        }

        #[test]
        fn test_multiple_frame() {
            let test = async {
                let stream = futures::stream::iter(vec![
                    FRAME_SOF, 1, 2, 3, FRAME_EOF, FRAME_SOF, 3, 2, 1, FRAME_EOF,
                ]);

                let frames_stream = to_frames(stream);

                let frames: Vec<_> = frames_stream.collect().await;

                assert_eq!(frames, vec![Ok(vec![1, 2, 3]), Ok(vec![3, 2, 1])]);
            };

            futures::executor::block_on(test)
        }

        #[test]
        fn test_recovery() {
            let test = async {
                let stream = futures::stream::iter(vec![
                    FRAME_SOF, 1, 2, 3, FRAME_SOF, 3, FRAME_SOF, 2, 1, FRAME_EOF,
                ]);

                let frames_stream = to_frames(stream);

                let frames: Vec<_> = frames_stream.collect().await;

                assert_eq!(
                    frames,
                    vec![
                        Err(FrameParsingErr::UnexpectedSof),
                        Err(FrameParsingErr::DataBeforeSof),
                        Ok(vec![2, 1])
                    ]
                );
            };

            futures::executor::block_on(test)
        }
    }

    pub struct RpcConn<'a> {
        low_level: Mutex<(
            Box<dyn futures::sink::Sink<Request, Error = transports::RpcErrorType> + Unpin + 'a>,
            Box<dyn futures::stream::Stream<Item = RequestResponse> + Unpin + 'a>,
            u32,
        )>,
    }

    impl<'a> RpcConn<'a> {
        pub fn new<
            TSink: futures::sink::Sink<Request, Error = transports::RpcErrorType> + Unpin + 'a,
            TStream: futures::stream::Stream<Item = RequestResponse> + Unpin + 'a,
        >(
            sink: TSink,
            stream: TStream,
        ) -> Self {
            RpcConn {
                low_level: Mutex::new((Box::new(sink), Box::new(stream), 0)),
            }
        }

        pub async fn call(
            &mut self,
            mut req: Request,
        ) -> Result<RequestResponse, transports::RpcErrorType> {
            let mut locked = self.low_level.lock().await;

            let request_id = locked.2;
            req.request_id = request_id;

            locked.2 = req.request_id + 1;

            locked.0.send(req).await;

            if let Some(resp) = locked.1.next().await {
                if resp.request_id == request_id {
                    return Ok(resp);
                } else {
                    // Todo: Handle this failure mode.
                    return Ok(resp);
                }
            }

            // TODO: Verify request ID matches in the response.
            Err(transports::RpcErrorType::ConnectionFailedErr)
        }
    }

    async fn response_demux<
        TResponseStream: futures::stream::Stream<Item = Response>,
        TRequestResponseSink: futures::sink::Sink<RequestResponse, Error = SendError> + Unpin,
        TNotificationSink: futures::sink::Sink<Notification, Error = SendError> + Unpin,
    >(
        stream: TResponseStream,
        mut req_resp_sink: TRequestResponseSink,
        mut notif_sink: TNotificationSink,
    ) -> () {
        let mut stream = Box::pin(stream);
        let req_resp_sink = &mut req_resp_sink;
        let notif_sink = &mut notif_sink;
        while let Some(resp) = stream.next().await {
            match resp.r#type {
                Some(Type::RequestResponse(req_resp)) => req_resp_sink.send(req_resp).await,
                Some(Type::Notification(notif)) => notif_sink.send(notif).await,
                _ => panic!("No oneof set!"),
            };
        }
    }

    pub async fn set_up_connection<
        'a,
        TSink: futures::sink::Sink<Request, Error = transports::RpcErrorType> + Unpin + 'a,
        TStream: futures::stream::Stream<Item = Response> + 'a,
    >(
        conn: transports::Connection<TSink, TStream>,
    ) -> Result<(RpcConn<'a>, impl Future<Output = ()>), crate::rpc::transports::RpcErrorType> {
        let transports::Connection { sink, stream } = conn;

        let (notif_sink, mut _notif_stream) = futures::channel::mpsc::unbounded();
        let (req_resp_sink, req_resp_stream) = futures::channel::mpsc::unbounded();

        let resp_demux = response_demux(stream, req_resp_sink, notif_sink);

        let rpc_conn = RpcConn::new(sink, req_resp_stream);

        Ok((rpc_conn, resp_demux))
    }

    pub mod transports {
        use crate::messages::zmk::{Request, Response};
        use std::fmt;

        use futures::channel::mpsc::SendError;

        pub struct Connection<
            TSink: futures::sink::Sink<Request>,
            TStream: futures::stream::Stream<Item = Response>,
        > {
            pub sink: TSink,
            pub stream: TStream,
        }

        #[derive(Debug, Clone)]
        pub enum RpcErrorType {
            ConnectionFailedErr,
            MessageEncodingFailed(prost::EncodeError),
            MessageDecodingFailed,
        }

        impl std::error::Error for RpcErrorType {}

        impl From<prost::EncodeError> for RpcErrorType {
            fn from(value: prost::EncodeError) -> Self {
                RpcErrorType::MessageEncodingFailed(value)
            }
        }

        impl From<SendError> for RpcErrorType {
            fn from(value: SendError) -> Self {
                RpcErrorType::ConnectionFailedErr
            }
        }

        impl fmt::Display for RpcErrorType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "display implementation becomes the error message")
            }
        }
    }
}
