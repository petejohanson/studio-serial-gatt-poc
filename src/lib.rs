use std::fmt;
use std::rc::Rc;

use futures::future::{join};
use futures::lock::Mutex;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

use web_sys::console;

// use cddl_lib::*;
use prost;

use futures::channel::mpsc::*;
use futures::{SinkExt, StreamExt};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));
}
pub mod rpc {
    // use cddl_lib::{Notification, Request, RequestResponse, Response};
    use crate::messages::zmk::{Request, RequestResponse, Response, Notification, response::Type};
    use futures::{channel::mpsc::SendError, lock::Mutex, Future, SinkExt, StreamExt};
    use wasm_bindgen::JsValue;
    use web_sys::console;

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
                    console::log_1(&JsValue::from("Out of sync!"));
                    return Ok(resp);
                }
            }
            console::log_1(&JsValue::from("No response message!"));
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
            console::log_1(&JsValue::from("Got a response"));
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

        pub mod web_ble {
            // use cddl_lib::{Request, Response};
            use crate::messages::zmk::{Request, Response};
            use prost::Message;
            use futures::channel::mpsc::unbounded;
            use futures::executor::block_on;
            use futures::stream;
            use futures::FutureExt;
            use futures::SinkExt;
            use js_sys::Uint8Array;
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            use wasm_bindgen::JsValue;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{console, BluetoothRemoteGattCharacteristic};

            use crate::rpc;

            use super::Connection;
            use super::RpcErrorType;

            const SERVICE_UUID: &str = "00000000-0196-6107-c967-c5cfb1c2482a";
            const RPC_CHRC_UUID: &str = "00000001-0196-6107-c967-c5cfb1c2482a";

            struct GattRequestSink {
                chrc: BluetoothRemoteGattCharacteristic,
                pending_send: Option<JsFuture>,
            }

            impl GattRequestSink {
                fn new(chrc: BluetoothRemoteGattCharacteristic) -> Self {
                    GattRequestSink {
                        chrc,
                        pending_send: None,
                    }
                }
            }

            impl<'a> futures::sink::Sink<Request> for GattRequestSink {
                type Error = super::RpcErrorType;

                fn poll_ready(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<Result<(), Self::Error>> {
                    self.get_mut()
                        .pending_send
                        .as_mut()
                        .map(|send| {
                            send.poll_unpin(cx)
                                .map_ok(|_| ())
                                .map_err(|_| super::RpcErrorType::ConnectionFailedErr)
                        })
                        .unwrap_or(std::task::Poll::Ready(Ok(())))
                }

                fn start_send(
                    self: std::pin::Pin<&mut Self>,
                    item: Request,
                ) -> Result<(), Self::Error> {
                    let mut buf = Vec::with_capacity(200);
                    item.encode(&mut buf);

                    // let bytes = cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&item);

                    let mut framed_bytes = crate::rpc::framing::framed(buf);

                    let arr = js_sys::Uint8Array::new_with_length(framed_bytes.len() as u32);
                    arr.copy_from(&framed_bytes);

                    let len = framed_bytes.len();

                    let body: &mut [u8] = &mut framed_bytes[0..len];

                    self.get_mut().pending_send = Some(wasm_bindgen_futures::JsFuture::from(
                        self.chrc.write_value_with_u8_array(body),
                    ));

                    Ok(())
                }

                fn poll_flush(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<Result<(), Self::Error>> {
                    std::task::Poll::Ready(Ok(()))
                }

                fn poll_close(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<Result<(), Self::Error>> {
                    std::task::Poll::Ready(Ok(()))
                }
            }

            fn get_gatt_response_stream(
                chrc: BluetoothRemoteGattCharacteristic,
            ) -> impl futures::stream::Stream<Item = Response> {
                let (mut sink, stream) = unbounded();

                chrc.start_notifications();
                let chrc_cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
                    let char = e
                        .target()
                        .expect("Target of the characteristic changed")
                        .dyn_ref::<BluetoothRemoteGattCharacteristic>()
                        .unwrap()
                        .clone();

                    let val = Uint8Array::new(&char.value().expect("Have a value!").buffer());

                    let val = val.to_vec();

                    let (msg_bytes, _remainder) = rpc::framing::parse_frame(val);

                    let response_msg: Result<Response, _> = Response::decode(&mut msg_bytes.as_slice());
                    // let response_msg: Result<Response, _> =
                    //     cddl_lib::serialization::Deserialize::from_cbor_bytes(&msg_bytes);

                    if let Ok(resp) = response_msg {
                        block_on(sink.send(resp));
                    }
                }) as Box<dyn FnMut(_)>);

                chrc.add_event_listener_with_callback(
                    "characteristicvaluechanged",
                    &chrc_cb.as_ref().unchecked_ref(),
                );
                chrc_cb.forget();

                return stream;
            }

            pub async fn get_ble_connection() -> Result<
                Connection<
                    impl futures::sink::Sink<Request, Error = RpcErrorType>,
                    impl futures::stream::Stream<Item = Response>,
                >,
                RpcErrorType,
            > {
                let bt = web_sys::window()
                    .unwrap()
                    .navigator()
                    .bluetooth()
                    .expect("Have BT available");

                let mut opts = web_sys::RequestDeviceOptions::new();
                opts.accept_all_devices(true);
                opts.optional_services(&JsValue::from(
                    [JsValue::from_str(SERVICE_UUID)]
                        .iter()
                        .collect::<js_sys::Array>(),
                ));
                let js_dev = wasm_bindgen_futures::JsFuture::from(bt.request_device(&opts))
                    .await
                    .expect("Got a device");

                console::log_1(&js_dev);
                let dev = web_sys::BluetoothDevice::from(js_dev);
                let gatt = dev.gatt().expect("GATT is there!");
                let _connected = wasm_bindgen_futures::JsFuture::from(gatt.connect()).await;
                console::log_1(&JsValue::from("Connected to GATT"));
                let js_svc = wasm_bindgen_futures::JsFuture::from(
                    gatt.get_primary_service_with_str(SERVICE_UUID),
                )
                .await
                .expect("Found the service");

                console::log_1(&js_svc);

                let svc = web_sys::BluetoothRemoteGattService::from(js_svc);

                let js_chrc = wasm_bindgen_futures::JsFuture::from(
                    svc.get_characteristic_with_str(RPC_CHRC_UUID),
                )
                .await
                .expect("Found the chrc");

                console::log_1(&js_chrc);

                let chrc = BluetoothRemoteGattCharacteristic::from(js_chrc);

                let sink = GattRequestSink::new(chrc.clone());
                let stream = get_gatt_response_stream(chrc.clone());

                Ok::<_, RpcErrorType>(Connection { sink, stream })
            }
        }

        pub mod web_serial {
            // use cddl_lib::{Request, Response};
            use crate::messages::zmk::{Request, Response};
            use prost::Message;
            use futures::stream;
            use futures::FutureExt;
            use wasm_bindgen::JsValue;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{console, ReadableStreamDefaultReader};

            use crate::rpc;

            pub async fn get_serial_connection() -> Result<
                super::Connection<
                    impl futures::sink::Sink<Request, Error = super::RpcErrorType>,
                    impl futures::stream::Stream<Item = Response>,
                >,
                super::RpcErrorType,
            > {
                let serial = web_sys::window().unwrap().navigator().serial();

                let opts = web_sys::SerialPortRequestOptions::new();
                let port =
                    wasm_bindgen_futures::JsFuture::from(serial.request_port_with_options(&opts))
                        .await
                        .unwrap();

                let open_opts = web_sys::SerialOptions::new(12500);
                let real_p = web_sys::SerialPort::from(port);

                let _opened = wasm_bindgen_futures::JsFuture::from(real_p.open(&open_opts))
                    .await
                    .unwrap();
                let writable = real_p.writable();
                let writer = writable.get_writer().unwrap();

                let readable = real_p.readable();
                let reader_obj = readable.get_reader();
                let reader = web_sys::ReadableStreamDefaultReader::from(JsValue::from(reader_obj));

                Ok(super::Connection {
                    sink: SerialSink::new(writer),
                    stream: response_stream(reader),
                })
            }

            struct SerialSink {
                writer: web_sys::WritableStreamDefaultWriter,
                pending_send: Option<JsFuture>,
            }

            impl SerialSink {
                fn new(writer: web_sys::WritableStreamDefaultWriter) -> Self {
                    SerialSink {
                        writer,
                        pending_send: None,
                    }
                }
            }

            impl<'a> futures::sink::Sink<Request> for SerialSink {
                type Error = super::RpcErrorType;

                fn poll_ready(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<Result<(), Self::Error>> {
                    self.get_mut()
                        .pending_send
                        .as_mut()
                        .map(|send| {
                            send.poll_unpin(cx)
                                .map_ok(|_| ())
                                .map_err(|_| super::RpcErrorType::ConnectionFailedErr)
                        })
                        .unwrap_or(std::task::Poll::Ready(Ok(())))
                }

                fn start_send(
                    self: std::pin::Pin<&mut Self>,
                    item: Request,
                ) -> Result<(), Self::Error> {
                    // let bytes = cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&item);
                    let mut buf = Vec::with_capacity(200);
                    item.encode(&mut buf);

                    let framed_bytes = crate::rpc::framing::framed(buf);

                    let arr = js_sys::Uint8Array::new_with_length(framed_bytes.len() as u32);
                    arr.copy_from(&framed_bytes);

                    self.get_mut().pending_send = Some(wasm_bindgen_futures::JsFuture::from(
                        self.writer.write_with_chunk(&JsValue::from(arr)),
                    ));

                    Ok(())
                }

                fn poll_flush(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<Result<(), Self::Error>> {
                    std::task::Poll::Ready(Ok(()))
                }

                fn poll_close(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<Result<(), Self::Error>> {
                    std::task::Poll::Ready(Ok(()))
                }
            }

            async fn next_read(
                r: ReadableStreamDefaultReader,
            ) -> Option<(Response, ReadableStreamDefaultReader)> {
                console::log_1(&JsValue::from("Reading from the reader"));
                let read_resp: JsValue = wasm_bindgen_futures::JsFuture::from(r.read())
                    .await
                    .unwrap();

                console::log_1(&JsValue::from("Got a read response from the reader"));
                // let done = js_sys::Reflect::get(&read_resp, &JsValue::from("done")).unwrap();
                let val = js_sys::Uint8Array::from(
                    js_sys::Reflect::get(&read_resp, &JsValue::from("value")).unwrap(),
                );

                let val_vec = val.to_vec();

                let (msg_bytes, _remainder) = rpc::framing::parse_frame(val_vec);

                let response_msg: Result<Response, _> = Response::decode(&mut msg_bytes.as_slice());
                // let response_msg: Result<Response, _> =
                //     cddl_lib::serialization::Deserialize::from_cbor_bytes(&msg_bytes);

                Some((response_msg.expect("Got a message"), r))
            }

            fn response_stream(
                reader: web_sys::ReadableStreamDefaultReader,
            ) -> impl futures::stream::Stream<Item = Response> {
                stream::unfold(reader, next_read)
            }
        }
    }
}

async fn test_rpc_conn<'a>(mut rpc_conn: rpc::RpcConn<'a>, demux: impl Future<Output = ()>) -> () {
    let caller = async move {
        // let req_get_lock_state = crate::messages::zmk::Request { request_id: 123, subsystem: Some(crate::messages::zmk::request::Subsystem::Core(crate::messages::zmk::core::Request { request_type: Some(crate::messages::zmk::core::request::RequestType::GetLockStatus(true)) }))};
        // let locked = rpc_conn.call(req_get_lock_state).await.map(|crate::messages::zmk::RequestResponse { .. }| {
        //     console::log_1(&JsValue::from("GOT RESPONSE"));
        // });
        let req_list_all = crate::messages::zmk::Request { request_id: 123, subsystem: Some(crate::messages::zmk::request::Subsystem::Behaviors(crate::messages::zmk::behaviors::Request { request_type: Some(crate::messages::zmk::behaviors::request::RequestType::ListAllBehaviors(true)) }))};

        let list = rpc_conn.call(req_list_all).await.map(|crate::messages::zmk::RequestResponse { subsystem, .. }| {
            match subsystem {
                Some(crate::messages::zmk::request_response::Subsystem::Behaviors(crate::messages::zmk::behaviors::Response {
                    response_type: Some(crate::messages::zmk::behaviors::response::ResponseType::ListAllBehaviors(r))
                })) => {
                    console::log_1(&JsValue::from("Got the behaviors!"));
                    Some(r.behaviors)
                }
                _ => {
                    console::log_1(&JsValue::from("Some other response"));
                    None
                },
            }
        }).expect("The RPC suceeded").expect("Got the right response type");

        let mut details = vec![];
         let mut summary_iter = list.into_iter();
         while let Some(s) = summary_iter.next() {
             let req = crate::messages::zmk::Request { request_id: 123, subsystem: Some(crate::messages::zmk::request::Subsystem::Behaviors(crate::messages::zmk::behaviors::Request { request_type: Some(crate::messages::zmk::behaviors::request::RequestType::GetBehaviorDetails(crate::messages::zmk::behaviors::GetBehaviorDetailsRequest { behavior_id: s })) }))};
            

            if let Ok(crate::messages::zmk::RequestResponse { subsystem, .. }) = rpc_conn.call(req).await {
                match subsystem {
                    Some(crate::messages::zmk::request_response::Subsystem::Behaviors(crate::messages::zmk::behaviors::Response {
                        response_type: Some(crate::messages::zmk::behaviors::response::ResponseType::GetBehaviorDetails(r))
                    })) => details.push(r),
                    _ => {
                        console::log_1(&JsValue::from("Some other response"));
                        ()
                    },
                }
            }
        
        }

        let behaviors = itertools::join(details.into_iter().map(|d| d.friendly_name), ", ");
        console::log_1(&JsValue::from(format!("Got behaviors: {behaviors}")));
    };

    join(caller, demux).await;
}

async fn test_ble() -> () {
    let conn = rpc::transports::web_ble::get_ble_connection()
        .await
        .unwrap();

    let (rpc_conn, demux) = rpc::set_up_connection(conn).await.unwrap();
    test_rpc_conn(rpc_conn, demux).await;
}

async fn test_serial() -> () {
    let conn = rpc::transports::web_serial::get_serial_connection()
        .await
        .unwrap();

    let (rpc_conn, demux) = rpc::set_up_connection(conn).await.unwrap();

    test_rpc_conn(rpc_conn, demux).await;
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let ble_button = document
        .query_selector("#ble_button")
        .unwrap()
        .expect("should have a button with that ID");

    let ble_cb = Closure::wrap(Box::new(|e: web_sys::Event| {
        wasm_bindgen_futures::spawn_local(test_ble());
    }) as Box<dyn FnMut(_)>);

    ble_button.add_event_listener_with_callback("click", &ble_cb.as_ref().unchecked_ref());
    ble_cb.forget();

    let serial_button = document
        .query_selector("#serial_button")
        .unwrap()
        .expect("should have a button with that ID");

    let serial_cb = Closure::wrap(Box::new(|e: web_sys::Event| {
        wasm_bindgen_futures::spawn_local(test_serial());
    }) as Box<dyn FnMut(_)>);

    serial_button.add_event_listener_with_callback("click", &serial_cb.as_ref().unchecked_ref());
    serial_cb.forget();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
