
use futures::stream;
use futures::stream::StreamExt;
use futures::FutureExt;
use prost::Message;
use wasm_bindgen_futures::JsFuture;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{console, ReadableStreamDefaultReader};
use zmk_studio_rpc::messages::zmk::{Request, Response};

use zmk_studio_rpc::rpc::framing;
use zmk_studio_rpc::rpc::transports::{
    get_response_stream_from_framed_bytes, Connection, RpcErrorType,
};

pub async fn get_connection() -> Result<
    Connection<
        impl futures::sink::Sink<Request, Error = RpcErrorType>,
        impl futures::stream::Stream<Item = Response>,
    >,
    RpcErrorType,
> {
    let serial = web_sys::window().unwrap().navigator().serial();

    let opts = web_sys::SerialPortRequestOptions::new();
    let port = wasm_bindgen_futures::JsFuture::from(serial.request_port_with_options(&opts))
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

    Ok(Connection {
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
    type Error = RpcErrorType;

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
                    .map_err(|_| RpcErrorType::ConnectionFailedErr)
            })
            .unwrap_or(std::task::Poll::Ready(Ok(())))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: Request) -> Result<(), Self::Error> {
        let mut buf = Vec::with_capacity(200);
        item.encode(&mut buf)?;

        let framed_bytes = framing::framed(buf);

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
) -> Option<(Vec<u8>, ReadableStreamDefaultReader)> {
    console::log_1(&JsValue::from("Reading from the reader"));
    let read_resp: JsValue = wasm_bindgen_futures::JsFuture::from(r.read())
        .await
        .unwrap();

    let val = js_sys::Uint8Array::from(
        js_sys::Reflect::get(&read_resp, &JsValue::from("value")).unwrap(),
    );

    Some((val.to_vec(), r))
}

fn response_stream<'a>(
    reader: web_sys::ReadableStreamDefaultReader,
) -> impl futures::stream::Stream<Item = Response> + 'a {
    let byte_stream = Box::pin(stream::unfold(reader, next_read).flat_map(|v| stream::iter(v)));

    get_response_stream_from_framed_bytes(byte_stream)
}
