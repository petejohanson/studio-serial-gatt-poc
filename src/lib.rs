use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys::console;
use web_sys::Navigator;
use itertools::Itertools;

use cddl_lib_wasm::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn is_escaped_byte(b: u8) -> bool {
    match b {
        0xAB | 0xAC | 0xAD => true,
        _ => false,
    }
}

fn framed(payload: Vec<u8>) -> Vec<u8> {
    let escaped = payload.into_iter().flat_map(|b| 
        if is_escaped_byte(b) {
            vec![0xAC, b].into_iter()
        } else {
            vec![b].into_iter()
        }
    );

    return std::iter::once(0xAB).chain(escaped).chain(std::iter::once(0xAD)).collect();
}

const SERVICE_UUID: &str = "00000000-0196-6107-c967-c5cfb1c2482a";
const RPC_CHRC_UUID: &str = "00000001-0196-6107-c967-c5cfb1c2482a";

async fn test_ble() -> () {
    let bt = web_sys::window().unwrap().navigator().bluetooth().expect("Have BT available");

    let mut opts = web_sys::RequestDeviceOptions::new();
    opts.accept_all_devices(true);
    opts.optional_services(&JsValue::from([JsValue::from_str(SERVICE_UUID)].into_iter().collect::<js_sys::Array>()));
    let js_dev = wasm_bindgen_futures::JsFuture::from(bt.request_device(&opts)).await.expect("Got a device");
    
    console::log_1(&js_dev);
    let dev = web_sys::BluetoothDevice::from(js_dev);
    let gatt = dev.gatt().expect("GATT is there!");
    let connected = wasm_bindgen_futures::JsFuture::from(gatt.connect()).await;
    console::log_1(&JsValue::from("Connected to GATT"));
    let js_svc = wasm_bindgen_futures::JsFuture::from(gatt.get_primary_service_with_str(SERVICE_UUID)).await.expect("Found the service");

    console::log_1(&js_svc);

    let svc = web_sys::BluetoothRemoteGattService::from(js_svc);

    let js_chrc = wasm_bindgen_futures::JsFuture::from(svc.get_characteristic_with_str(RPC_CHRC_UUID)).await.expect("Found the chrc");

    console::log_1(&js_chrc);

    let req = Request::new_core_subsystem(&CoreSubsystem::new(&CoreRequest::new_get_lock_state(&GetLockState::new())));
    let bytes = req.to_cbor_bytes();

    let mut framed_bytes = framed(bytes);

    let len = framed_bytes.len();

    let body: &mut [u8] = &mut framed_bytes[0..len];

    let chrc = web_sys::BluetoothRemoteGattCharacteristic::from(js_chrc);

    chrc.write_value_with_u8_array(body);
}

async fn test_serial() -> () {
    let serial = web_sys::window().unwrap().navigator().serial();

    let opts = web_sys::SerialPortRequestOptions::new();
    let port = wasm_bindgen_futures::JsFuture::from(serial.request_port_with_options(&opts)).await;

    match port {
        Ok(p) => {
            let open_opts = web_sys::SerialOptions::new(12500);
            let real_p = web_sys::SerialPort::from(p);
            let req = Request::new_core_subsystem(&CoreSubsystem::new(&CoreRequest::new_get_lock_state(&GetLockState::new())));
            let bytes = req.to_cbor_bytes();

            let framed_bytes = framed(bytes);

            let str = format!("{:02x}", framed_bytes.iter().format(" "));
            console::log_1(&JsValue::from(str));

            let arr = js_sys::Uint8Array::new_with_length(framed_bytes.len() as u32);
            arr.copy_from(&framed_bytes);
            console::log_1(&real_p);
            console::log_1(&arr);

            let opened = wasm_bindgen_futures::JsFuture::from(real_p.open(&open_opts)).await.unwrap();
            console::log_1(&opened);
            let writable = real_p.writable();
            let writer = writable.get_writer().unwrap();

            wasm_bindgen_futures::JsFuture::from(writer.write_with_chunk(&JsValue::from(arr))).await.unwrap();
            console::log_1(&JsValue::from("Wrote the msg!"));
        },
        _ => {
        }
    }
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
    let button = document.query_selector("#button").unwrap().expect("should have a button with that ID");

    let cb = Closure::wrap(Box::new(|e: web_sys::Event| {
        wasm_bindgen_futures::spawn_local(test_ble());
    }) as Box<dyn FnMut(_)>);

    button.add_event_listener_with_callback("click", &cb.as_ref().unchecked_ref());
    cb.forget();



    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
