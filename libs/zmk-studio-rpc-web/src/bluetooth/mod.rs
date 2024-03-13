use futures::channel::mpsc::unbounded;
use futures::executor::block_on;
use futures::FutureExt;
use futures::SinkExt;
use js_sys::Uint8Array;
use prost::Message;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, BluetoothRemoteGattCharacteristic};
use zmk_studio_rpc::messages::zmk::{Request, Response};

use zmk_studio_rpc::rpc::framing;
use zmk_studio_rpc::rpc::transports::{Connection, RpcErrorType};

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
        item.encode(&mut buf);

        let mut framed_bytes = framing::framed(buf);

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

        let (msg_bytes, _remainder) = framing::parse_frame(val);

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

pub async fn get_connection() -> Result<
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
    let js_svc =
        wasm_bindgen_futures::JsFuture::from(gatt.get_primary_service_with_str(SERVICE_UUID))
            .await
            .expect("Found the service");

    console::log_1(&js_svc);

    let svc = web_sys::BluetoothRemoteGattService::from(js_svc);

    let js_chrc =
        wasm_bindgen_futures::JsFuture::from(svc.get_characteristic_with_str(RPC_CHRC_UUID))
            .await
            .expect("Found the chrc");

    console::log_1(&js_chrc);

    let chrc = BluetoothRemoteGattCharacteristic::from(js_chrc);

    let sink = GattRequestSink::new(chrc.clone());
    let stream = get_gatt_response_stream(chrc.clone());

    Ok::<_, RpcErrorType>(Connection { sink, stream })
}
