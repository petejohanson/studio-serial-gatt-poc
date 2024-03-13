use futures::future::join;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

use web_sys::console;

use zmk_studio_rpc::rpc::RpcConn;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

async fn test_rpc_conn<'a>(mut rpc_conn: RpcConn<'a>, demux: impl Future<Output = ()>) -> () {
    let caller = async move {
        let req_list_all = zmk_studio_rpc::messages::zmk::Request { request_id: 123, subsystem: Some(zmk_studio_rpc::messages::zmk::request::Subsystem::Behaviors(zmk_studio_rpc::messages::zmk::behaviors::Request { request_type: Some(zmk_studio_rpc::messages::zmk::behaviors::request::RequestType::ListAllBehaviors(true)) }))};

        let list = rpc_conn.call(req_list_all).await.map(|zmk_studio_rpc::messages::zmk::RequestResponse { subsystem, .. }| {
            match subsystem {
                Some(zmk_studio_rpc::messages::zmk::request_response::Subsystem::Behaviors(zmk_studio_rpc::messages::zmk::behaviors::Response {
                    response_type: Some(zmk_studio_rpc::messages::zmk::behaviors::response::ResponseType::ListAllBehaviors(r))
                })) => {
                    console::log_1(&JsValue::from("Got the behaviors!"));
                    Some(r.behaviors)
                }
                _ => {
                    console::log_1(&JsValue::from("Some other response"));
                    None
                },
            }
        }).expect("The RPC succeeded").expect("Got the right response type");

        let mut details = vec![];
        let mut summary_iter = list.into_iter();
        while let Some(s) = summary_iter.next() {
            let req = zmk_studio_rpc::messages::zmk::Request { request_id: 123, subsystem: Some(zmk_studio_rpc::messages::zmk::request::Subsystem::Behaviors(zmk_studio_rpc::messages::zmk::behaviors::Request { request_type: Some(zmk_studio_rpc::messages::zmk::behaviors::request::RequestType::GetBehaviorDetails(zmk_studio_rpc::messages::zmk::behaviors::GetBehaviorDetailsRequest { behavior_id: s })) }))};

            if let Ok(zmk_studio_rpc::messages::zmk::RequestResponse { subsystem, .. }) =
                rpc_conn.call(req).await
            {
                match subsystem {
                    Some(zmk_studio_rpc::messages::zmk::request_response::Subsystem::Behaviors(zmk_studio_rpc::messages::zmk::behaviors::Response {
                        response_type: Some(zmk_studio_rpc::messages::zmk::behaviors::response::ResponseType::GetBehaviorDetails(r))
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
    let conn = zmk_studio_rpc_web::bluetooth::get_connection()
        .await
        .unwrap();

    let (rpc_conn, demux) = zmk_studio_rpc::rpc::set_up_connection(conn).await.unwrap();
    test_rpc_conn(rpc_conn, demux).await;
}

async fn test_serial() -> () {
    let conn = zmk_studio_rpc_web::serial::get_connection().await.unwrap();

    let (rpc_conn, demux) = zmk_studio_rpc::rpc::set_up_connection(conn).await.unwrap();

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
