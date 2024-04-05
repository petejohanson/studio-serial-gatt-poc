use futures::future::join_all;
use futures::StreamExt;
use std::{convert::TryFrom, future::Future};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

use web_sys::console;

use zmk_studio_rpc::{
    messages::zmk::{
        behaviors::{
            get_behavior_details_response::ParametersType, BehaviorBindingParameterStandardDomain
        },
        core::notification::NotificationType,
        Notification,
    },
    rpc::RpcConn,
};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

async fn test_rpc_conn<'a>(
    mut rpc_conn: RpcConn<'a>,
    demux: impl Future<Output = ()>,
    notif_stream: impl futures::Stream<Item = zmk_studio_rpc::messages::zmk::Notification>,
) -> () {
    let caller = async move {
        // let unlock_req = zmk_studio_rpc::messages::zmk::Request { request_id: 0, subsystem: Some(zmk_studio_rpc::messages::zmk::request::Subsystem::Core(zmk_studio_rpc::messages::zmk::core::Request { request_type: Some(zmk_studio_rpc::messages::zmk::core::request::RequestType::RequestUnlock(true)) }))};
        // let resp = rpc_conn.call(unlock_req).await;
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
            } else {
                console::log_1(&JsValue::from(format!("Got an error! {s}")));
            }
        }

        let render_param = |p: i32| match BehaviorBindingParameterStandardDomain::try_from(p) {
            Ok(BehaviorBindingParameterStandardDomain::Nil) => "Nil",
            Ok(BehaviorBindingParameterStandardDomain::HidUsage) => "HID Usage",
            Ok(BehaviorBindingParameterStandardDomain::LayerIndex) => "Layer Index",
            Ok(BehaviorBindingParameterStandardDomain::HsvValue) => "HSV Value",
            Err(_) => "None",
        };

        let behaviors = itertools::join(
            details.into_iter().map(|d| {
                let n = d.friendly_name;

                let (p1, p2) = if let Some(p
                ) = d.parameters_type
                {
                    match p {
                        ParametersType::Standard(s) => (
                            render_param(s.param1),
                            render_param(s.param2),
                        ),
                        ParametersType::Custom(c) => {
                            for set in c.param_sets.iter() {
                                let len = set.param1.len();
                                for p in set.param1.iter() {
                                    let name = &p.name;
                                    console::log_1(&JsValue::from(format!("param1 {name}")));
                                }

                                for p in set.param2.iter() {
                                    let name = &p.name;
                                    console::log_1(&JsValue::from(format!("param2 {name}")));
                                }
                            }
                            ("Custom", "Custom")
                        }
                    }
                } else {
                    ("None", "None")
                };

                format!("Behavior: {n}, param1: {p1}, param2: {p2}")
            }),
            "\n",
        );
        console::log_1(&JsValue::from(format!("Got behaviors:\n{behaviors}")));
    };

    let notification_consumer = async {
        let mut notif_stream = Box::pin(notif_stream);
        while let n = notif_stream.next().await {
            console::log_1(&JsValue::from("Got a notification"));
            match n {
                Some(Notification {
                    subsystem:
                        Some(zmk_studio_rpc::messages::zmk::notification::Subsystem::Core(
                            zmk_studio_rpc::messages::zmk::core::Notification {
                                notification_type: Some(NotificationType::LockStateChanged(ls)),
                            },
                        )),
                }) => console::log_1(&JsValue::from(format!("Lock state {ls}"))),
                _ => console::log_1(&JsValue::from("Some other notification")),
            }
        }
    };
    let futures: Vec<std::pin::Pin<Box<Future<Output = ()>>>> = vec![
        Box::pin(caller),
        Box::pin(demux),
        Box::pin(notification_consumer),
    ];

    join_all(futures.into_iter()).await;
}

async fn test_ble() -> () {
    let conn = zmk_studio_rpc_web::bluetooth::get_connection()
        .await
        .unwrap();

    let (rpc_conn, demux, notif_stream) =
        zmk_studio_rpc::rpc::set_up_connection(conn).await.unwrap();
    test_rpc_conn(rpc_conn, demux, notif_stream).await;
}

async fn test_serial() -> () {
    let conn = zmk_studio_rpc_web::serial::get_connection().await.unwrap();

    let (rpc_conn, demux, notif_stream) =
        zmk_studio_rpc::rpc::set_up_connection(conn).await.unwrap();

    test_rpc_conn(rpc_conn, demux, notif_stream).await;
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
