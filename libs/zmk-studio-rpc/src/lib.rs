
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));
}

pub mod rpc {
    use crate::messages::zmk::{Request, RequestResponse, Response, Notification, response::Type};
    use futures::{channel::mpsc::SendError, lock::Mutex, Future, SinkExt, StreamExt};

    pub mod framing {
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

        enum FramingParseState {
            FramingStateIdle,
            FramingStateAwaitingData,
            FramingStateEscaped,
            FramingStateErr,
            FramingStateEof,
        }

        const FRAME_SOF: u8 = 0xAB;
        const FRAME_ESC: u8 = 0xAC;
        const FRAME_EOF: u8 = 0xAD;

        pub fn parse_frame(vec: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
            let mut state = FramingParseState::FramingStateIdle;
            let mut dest: Vec<u8> = Vec::new();
            let mut rest: Vec<u8> = Vec::new();

            for v in vec.into_iter() {
                match state {
                    FramingParseState::FramingStateIdle => {
                        // TODO: Stop if we already got a frame
                        match v {
                            FRAME_SOF => state = FramingParseState::FramingStateAwaitingData,
                            _ => state = FramingParseState::FramingStateErr,
                        };
                    }
                    FramingParseState::FramingStateAwaitingData => {
                        match v {
                            FRAME_ESC => state = FramingParseState::FramingStateEscaped,
                            FRAME_EOF => state = FramingParseState::FramingStateEof,
                            data_byte => dest.push(data_byte),
                        };
                    }
                    FramingParseState::FramingStateEscaped => {
                        dest.push(v);
                    }
                    FramingParseState::FramingStateErr => match v {
                        FRAME_SOF => state = FramingParseState::FramingStateIdle,
                        _ => (),
                    },
                    FramingParseState::FramingStateEof => {
                        rest.push(v);
                    }
                }
            }
            return (dest, rest);
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
        use std::fmt;
        use crate::messages::zmk::{Request, Response};

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
        }

        impl std::error::Error for RpcErrorType {}

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
