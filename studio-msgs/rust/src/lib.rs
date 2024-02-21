#![allow(clippy::too_many_arguments)]

pub mod error;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;

use crate::error::*;
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub enum BehaviorRequest {
    ListAllBehaviors(ListAllBehaviors),
    GetBehaviorDetails(GetBehaviorDetails),
}

impl BehaviorRequest {
    pub fn new_list_all_behaviors(list_all_behaviors: ListAllBehaviors) -> Self {
        Self::ListAllBehaviors(list_all_behaviors)
    }

    pub fn new_get_behavior_details(get_behavior_details: GetBehaviorDetails) -> Self {
        Self::GetBehaviorDetails(get_behavior_details)
    }
}

#[derive(Clone, Debug)]
pub struct BehaviorResponse {
    pub list_all_behaviors_response: ListAllBehaviorsResponse,
}

impl BehaviorResponse {
    pub fn new(list_all_behaviors_response: ListAllBehaviorsResponse) -> Self {
        Self {
            list_all_behaviors_response,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BehaviorSubsystem {
    pub behavior_request: BehaviorRequest,
}

impl BehaviorSubsystem {
    pub fn new(behavior_request: BehaviorRequest) -> Self {
        Self { behavior_request }
    }
}

#[derive(Clone, Debug)]
pub struct BehaviorSummary {
    pub name: String,
}

impl BehaviorSummary {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug)]
pub enum CoreRequest {
    GetLockState(GetLockState),
    UnlockRequest(UnlockRequest),
    LockRequest(LockRequest),
}

impl CoreRequest {
    pub fn new_get_lock_state(get_lock_state: GetLockState) -> Self {
        Self::GetLockState(get_lock_state)
    }

    pub fn new_unlock_request(unlock_request: UnlockRequest) -> Self {
        Self::UnlockRequest(unlock_request)
    }

    pub fn new_lock_request(lock_request: LockRequest) -> Self {
        Self::LockRequest(lock_request)
    }
}

#[derive(Clone, Debug)]
pub struct CoreResponse {
    pub index_1: GetLockStateResponseOrUnlockResponseOrLockResponse,
}

impl CoreResponse {
    pub fn new(index_1: GetLockStateResponseOrUnlockResponseOrLockResponse) -> Self {
        Self { index_1 }
    }
}

#[derive(Clone, Debug)]
pub struct CoreSubsystem {
    pub core_request: CoreRequest,
}

impl CoreSubsystem {
    pub fn new(core_request: CoreRequest) -> Self {
        Self { core_request }
    }
}

#[derive(Clone, Debug)]
pub struct GetBehaviorDetails {
    pub get_behavior_details_payload: GetBehaviorDetailsPayload,
}

impl GetBehaviorDetails {
    pub fn new(get_behavior_details_payload: GetBehaviorDetailsPayload) -> Self {
        Self {
            get_behavior_details_payload,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GetBehaviorDetailsPayload {
    pub behavior_id: u64,
}

impl GetBehaviorDetailsPayload {
    pub fn new(behavior_id: u64) -> Self {
        Self { behavior_id }
    }
}

#[derive(Clone, Debug)]
pub struct GetLayersSummary;

impl GetLayersSummary {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GetLayersSummary {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct GetLayersSummaryResponse {
    pub keymap_layer_summary: KeymapLayerSummary,
}

impl GetLayersSummaryResponse {
    pub fn new(keymap_layer_summary: KeymapLayerSummary) -> Self {
        Self {
            keymap_layer_summary,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GetLockState;

impl GetLockState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GetLockState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct GetLockStateResponse {
    pub locked: bool,
}

impl GetLockStateResponse {
    pub fn new(locked: bool) -> Self {
        Self { locked }
    }
}

#[derive(Clone, Debug)]
pub enum GetLockStateResponseOrUnlockResponseOrLockResponse {
    GetLockStateResponse(GetLockStateResponse),
    UnlockResponse(UnlockResponse),
    LockResponse(LockResponse),
}

impl GetLockStateResponseOrUnlockResponseOrLockResponse {
    pub fn new_get_lock_state_response(get_lock_state_response: GetLockStateResponse) -> Self {
        Self::GetLockStateResponse(get_lock_state_response)
    }

    pub fn new_unlock_response(unlock_response: UnlockResponse) -> Self {
        Self::UnlockResponse(unlock_response)
    }

    pub fn new_lock_response(lock_response: LockResponse) -> Self {
        Self::LockResponse(lock_response)
    }
}

#[derive(Clone, Debug)]
pub struct KeymapLayerSummary {
    pub name: String,
    pub enabled: bool,
}

impl KeymapLayerSummary {
    pub fn new(name: String, enabled: bool) -> Self {
        Self { name, enabled }
    }
}

pub type KeymapRequest = GetLayersSummary;

#[derive(Clone, Debug)]
pub struct KeymapResponse {
    pub get_layers_summary_response: GetLayersSummaryResponse,
}

impl KeymapResponse {
    pub fn new(get_layers_summary_response: GetLayersSummaryResponse) -> Self {
        Self {
            get_layers_summary_response,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeymapSubsystem {
    pub keymap_request: KeymapRequest,
}

impl KeymapSubsystem {
    pub fn new(keymap_request: KeymapRequest) -> Self {
        Self { keymap_request }
    }
}

#[derive(Clone, Debug)]
pub struct ListAllBehaviors;

impl ListAllBehaviors {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ListAllBehaviors {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct ListAllBehaviorsResponse {
    pub behavior_summary: BehaviorSummary,
}

impl ListAllBehaviorsResponse {
    pub fn new(behavior_summary: BehaviorSummary) -> Self {
        Self { behavior_summary }
    }
}

#[derive(Clone, Debug)]
pub struct LockRequest;

impl LockRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LockRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct LockResponse;

impl LockResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LockResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Notification {
    pub notification_payload: NotificationPayload,
}

impl Notification {
    pub fn new(notification_payload: NotificationPayload) -> Self {
        Self {
            notification_payload,
        }
    }
}

pub type NotificationPayload = u64;

#[derive(Clone, Debug)]
pub enum Request {
    CoreSubsystem(CoreSubsystem),
    KeymapSubsystem(KeymapSubsystem),
    BehaviorSubsystem(BehaviorSubsystem),
}

impl Request {
    pub fn new_core_subsystem(core_subsystem: CoreSubsystem) -> Self {
        Self::CoreSubsystem(core_subsystem)
    }

    pub fn new_keymap_subsystem(keymap_subsystem: KeymapSubsystem) -> Self {
        Self::KeymapSubsystem(keymap_subsystem)
    }

    pub fn new_behavior_subsystem(behavior_subsystem: BehaviorSubsystem) -> Self {
        Self::BehaviorSubsystem(behavior_subsystem)
    }
}

#[derive(Clone, Debug)]
pub struct RequestResponse {
    pub response_payload: ResponsePayload,
}

impl RequestResponse {
    pub fn new(response_payload: ResponsePayload) -> Self {
        Self { response_payload }
    }
}

#[derive(Clone, Debug)]
pub enum Response {
    Notification(Notification),
    RequestResponse(RequestResponse),
}

impl Response {
    pub fn new_notification(notification: Notification) -> Self {
        Self::Notification(notification)
    }

    pub fn new_request_response(request_response: RequestResponse) -> Self {
        Self::RequestResponse(request_response)
    }
}

#[derive(Clone, Debug)]
pub enum ResponsePayload {
    CoreResponse(CoreResponse),
    KeymapResponse(KeymapResponse),
    BehaviorResponse(BehaviorResponse),
}

impl ResponsePayload {
    pub fn new_core_response(core_response: CoreResponse) -> Self {
        Self::CoreResponse(core_response)
    }

    pub fn new_keymap_response(keymap_response: KeymapResponse) -> Self {
        Self::KeymapResponse(keymap_response)
    }

    pub fn new_behavior_response(behavior_response: BehaviorResponse) -> Self {
        Self::BehaviorResponse(behavior_response)
    }
}

#[derive(Clone, Debug)]
pub struct UnlockRequest;

impl UnlockRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UnlockRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct UnlockResponse;

impl UnlockResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UnlockResponse {
    fn default() -> Self {
        Self::new()
    }
}
