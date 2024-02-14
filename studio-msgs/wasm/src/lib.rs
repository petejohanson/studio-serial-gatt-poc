#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BehaviorRequest(cddl_lib::BehaviorRequest);

#[wasm_bindgen]
impl BehaviorRequest {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BehaviorRequest, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new_list_all_behaviors(list_all_behaviors: &ListAllBehaviors) -> Self {
        Self(cddl_lib::BehaviorRequest::new_list_all_behaviors(
            list_all_behaviors.clone().into(),
        ))
    }

    pub fn new_get_behavior_details(get_behavior_details: &GetBehaviorDetails) -> Self {
        Self(cddl_lib::BehaviorRequest::new_get_behavior_details(
            get_behavior_details.clone().into(),
        ))
    }

    pub fn kind(&self) -> BehaviorRequestKind {
        match &self.0 {
            cddl_lib::BehaviorRequest::ListAllBehaviors(_) => BehaviorRequestKind::ListAllBehaviors,
            cddl_lib::BehaviorRequest::GetBehaviorDetails(_) => {
                BehaviorRequestKind::GetBehaviorDetails
            }
        }
    }

    pub fn as_list_all_behaviors(&self) -> Option<ListAllBehaviors> {
        match &self.0 {
            cddl_lib::BehaviorRequest::ListAllBehaviors(list_all_behaviors) => {
                Some(list_all_behaviors.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_get_behavior_details(&self) -> Option<GetBehaviorDetails> {
        match &self.0 {
            cddl_lib::BehaviorRequest::GetBehaviorDetails(get_behavior_details) => {
                Some(get_behavior_details.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cddl_lib::BehaviorRequest> for BehaviorRequest {
    fn from(native: cddl_lib::BehaviorRequest) -> Self {
        Self(native)
    }
}

impl From<BehaviorRequest> for cddl_lib::BehaviorRequest {
    fn from(wasm: BehaviorRequest) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::BehaviorRequest> for BehaviorRequest {
    fn as_ref(&self) -> &cddl_lib::BehaviorRequest {
        &self.0
    }
}

#[wasm_bindgen]
pub enum BehaviorRequestKind {
    ListAllBehaviors,
    GetBehaviorDetails,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BehaviorSubsystem(cddl_lib::BehaviorSubsystem);

#[wasm_bindgen]
impl BehaviorSubsystem {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BehaviorSubsystem, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn behavior_request(&self) -> BehaviorRequest {
        self.0.behavior_request.clone().into()
    }

    pub fn new(behavior_request: &BehaviorRequest) -> Self {
        Self(cddl_lib::BehaviorSubsystem::new(
            behavior_request.clone().into(),
        ))
    }
}

impl From<cddl_lib::BehaviorSubsystem> for BehaviorSubsystem {
    fn from(native: cddl_lib::BehaviorSubsystem) -> Self {
        Self(native)
    }
}

impl From<BehaviorSubsystem> for cddl_lib::BehaviorSubsystem {
    fn from(wasm: BehaviorSubsystem) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::BehaviorSubsystem> for BehaviorSubsystem {
    fn as_ref(&self) -> &cddl_lib::BehaviorSubsystem {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CoreRequest(cddl_lib::CoreRequest);

#[wasm_bindgen]
impl CoreRequest {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<CoreRequest, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new_get_lock_state(get_lock_state: &GetLockState) -> Self {
        Self(cddl_lib::CoreRequest::new_get_lock_state(
            get_lock_state.clone().into(),
        ))
    }

    pub fn new_unlock_request(unlock_request: &UnlockRequest) -> Self {
        Self(cddl_lib::CoreRequest::new_unlock_request(
            unlock_request.clone().into(),
        ))
    }

    pub fn new_lock_request(lock_request: &LockRequest) -> Self {
        Self(cddl_lib::CoreRequest::new_lock_request(
            lock_request.clone().into(),
        ))
    }

    pub fn kind(&self) -> CoreRequestKind {
        match &self.0 {
            cddl_lib::CoreRequest::GetLockState(_) => CoreRequestKind::GetLockState,
            cddl_lib::CoreRequest::UnlockRequest(_) => CoreRequestKind::UnlockRequest,
            cddl_lib::CoreRequest::LockRequest(_) => CoreRequestKind::LockRequest,
        }
    }

    pub fn as_get_lock_state(&self) -> Option<GetLockState> {
        match &self.0 {
            cddl_lib::CoreRequest::GetLockState(get_lock_state) => {
                Some(get_lock_state.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_unlock_request(&self) -> Option<UnlockRequest> {
        match &self.0 {
            cddl_lib::CoreRequest::UnlockRequest(unlock_request) => {
                Some(unlock_request.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_lock_request(&self) -> Option<LockRequest> {
        match &self.0 {
            cddl_lib::CoreRequest::LockRequest(lock_request) => Some(lock_request.clone().into()),
            _ => None,
        }
    }
}

impl From<cddl_lib::CoreRequest> for CoreRequest {
    fn from(native: cddl_lib::CoreRequest) -> Self {
        Self(native)
    }
}

impl From<CoreRequest> for cddl_lib::CoreRequest {
    fn from(wasm: CoreRequest) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::CoreRequest> for CoreRequest {
    fn as_ref(&self) -> &cddl_lib::CoreRequest {
        &self.0
    }
}

#[wasm_bindgen]
pub enum CoreRequestKind {
    GetLockState,
    UnlockRequest,
    LockRequest,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CoreSubsystem(cddl_lib::CoreSubsystem);

#[wasm_bindgen]
impl CoreSubsystem {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<CoreSubsystem, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn core_request(&self) -> CoreRequest {
        self.0.core_request.clone().into()
    }

    pub fn new(core_request: &CoreRequest) -> Self {
        Self(cddl_lib::CoreSubsystem::new(core_request.clone().into()))
    }
}

impl From<cddl_lib::CoreSubsystem> for CoreSubsystem {
    fn from(native: cddl_lib::CoreSubsystem) -> Self {
        Self(native)
    }
}

impl From<CoreSubsystem> for cddl_lib::CoreSubsystem {
    fn from(wasm: CoreSubsystem) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::CoreSubsystem> for CoreSubsystem {
    fn as_ref(&self) -> &cddl_lib::CoreSubsystem {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GetBehaviorDetails(cddl_lib::GetBehaviorDetails);

#[wasm_bindgen]
impl GetBehaviorDetails {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GetBehaviorDetails, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn get_behavior_details_payload(&self) -> GetBehaviorDetailsPayload {
        self.0.get_behavior_details_payload.clone().into()
    }

    pub fn new(get_behavior_details_payload: &GetBehaviorDetailsPayload) -> Self {
        Self(cddl_lib::GetBehaviorDetails::new(
            get_behavior_details_payload.clone().into(),
        ))
    }
}

impl From<cddl_lib::GetBehaviorDetails> for GetBehaviorDetails {
    fn from(native: cddl_lib::GetBehaviorDetails) -> Self {
        Self(native)
    }
}

impl From<GetBehaviorDetails> for cddl_lib::GetBehaviorDetails {
    fn from(wasm: GetBehaviorDetails) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetBehaviorDetails> for GetBehaviorDetails {
    fn as_ref(&self) -> &cddl_lib::GetBehaviorDetails {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GetBehaviorDetailsPayload(cddl_lib::GetBehaviorDetailsPayload);

#[wasm_bindgen]
impl GetBehaviorDetailsPayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GetBehaviorDetailsPayload, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn behavior_id(&self) -> u64 {
        self.0.behavior_id
    }

    pub fn new(behavior_id: u64) -> Self {
        Self(cddl_lib::GetBehaviorDetailsPayload::new(behavior_id))
    }
}

impl From<cddl_lib::GetBehaviorDetailsPayload> for GetBehaviorDetailsPayload {
    fn from(native: cddl_lib::GetBehaviorDetailsPayload) -> Self {
        Self(native)
    }
}

impl From<GetBehaviorDetailsPayload> for cddl_lib::GetBehaviorDetailsPayload {
    fn from(wasm: GetBehaviorDetailsPayload) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetBehaviorDetailsPayload> for GetBehaviorDetailsPayload {
    fn as_ref(&self) -> &cddl_lib::GetBehaviorDetailsPayload {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GetLayersSummary(cddl_lib::GetLayersSummary);

#[wasm_bindgen]
impl GetLayersSummary {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GetLayersSummary, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::GetLayersSummary::new())
    }
}

impl From<cddl_lib::GetLayersSummary> for GetLayersSummary {
    fn from(native: cddl_lib::GetLayersSummary) -> Self {
        Self(native)
    }
}

impl From<GetLayersSummary> for cddl_lib::GetLayersSummary {
    fn from(wasm: GetLayersSummary) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetLayersSummary> for GetLayersSummary {
    fn as_ref(&self) -> &cddl_lib::GetLayersSummary {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GetLockState(cddl_lib::GetLockState);

#[wasm_bindgen]
impl GetLockState {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GetLockState, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::GetLockState::new())
    }
}

impl From<cddl_lib::GetLockState> for GetLockState {
    fn from(native: cddl_lib::GetLockState) -> Self {
        Self(native)
    }
}

impl From<GetLockState> for cddl_lib::GetLockState {
    fn from(wasm: GetLockState) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetLockState> for GetLockState {
    fn as_ref(&self) -> &cddl_lib::GetLockState {
        &self.0
    }
}

pub type KeymapRequest = GetLayersSummary;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeymapSubsystem(cddl_lib::KeymapSubsystem);

#[wasm_bindgen]
impl KeymapSubsystem {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeymapSubsystem, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn keymap_request(&self) -> KeymapRequest {
        self.0.keymap_request.clone().into()
    }

    pub fn new(keymap_request: &KeymapRequest) -> Self {
        Self(cddl_lib::KeymapSubsystem::new(
            keymap_request.clone().into(),
        ))
    }
}

impl From<cddl_lib::KeymapSubsystem> for KeymapSubsystem {
    fn from(native: cddl_lib::KeymapSubsystem) -> Self {
        Self(native)
    }
}

impl From<KeymapSubsystem> for cddl_lib::KeymapSubsystem {
    fn from(wasm: KeymapSubsystem) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::KeymapSubsystem> for KeymapSubsystem {
    fn as_ref(&self) -> &cddl_lib::KeymapSubsystem {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ListAllBehaviors(cddl_lib::ListAllBehaviors);

#[wasm_bindgen]
impl ListAllBehaviors {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ListAllBehaviors, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::ListAllBehaviors::new())
    }
}

impl From<cddl_lib::ListAllBehaviors> for ListAllBehaviors {
    fn from(native: cddl_lib::ListAllBehaviors) -> Self {
        Self(native)
    }
}

impl From<ListAllBehaviors> for cddl_lib::ListAllBehaviors {
    fn from(wasm: ListAllBehaviors) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::ListAllBehaviors> for ListAllBehaviors {
    fn as_ref(&self) -> &cddl_lib::ListAllBehaviors {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LockRequest(cddl_lib::LockRequest);

#[wasm_bindgen]
impl LockRequest {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<LockRequest, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::LockRequest::new())
    }
}

impl From<cddl_lib::LockRequest> for LockRequest {
    fn from(native: cddl_lib::LockRequest) -> Self {
        Self(native)
    }
}

impl From<LockRequest> for cddl_lib::LockRequest {
    fn from(wasm: LockRequest) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::LockRequest> for LockRequest {
    fn as_ref(&self) -> &cddl_lib::LockRequest {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Request(cddl_lib::Request);

#[wasm_bindgen]
impl Request {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Request, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new_core_subsystem(core_subsystem: &CoreSubsystem) -> Self {
        Self(cddl_lib::Request::new_core_subsystem(
            core_subsystem.clone().into(),
        ))
    }

    pub fn new_keymap_subsystem(keymap_subsystem: &KeymapSubsystem) -> Self {
        Self(cddl_lib::Request::new_keymap_subsystem(
            keymap_subsystem.clone().into(),
        ))
    }

    pub fn new_behavior_subsystem(behavior_subsystem: &BehaviorSubsystem) -> Self {
        Self(cddl_lib::Request::new_behavior_subsystem(
            behavior_subsystem.clone().into(),
        ))
    }

    pub fn kind(&self) -> RequestKind {
        match &self.0 {
            cddl_lib::Request::CoreSubsystem(_) => RequestKind::CoreSubsystem,
            cddl_lib::Request::KeymapSubsystem(_) => RequestKind::KeymapSubsystem,
            cddl_lib::Request::BehaviorSubsystem(_) => RequestKind::BehaviorSubsystem,
        }
    }

    pub fn as_core_subsystem(&self) -> Option<CoreSubsystem> {
        match &self.0 {
            cddl_lib::Request::CoreSubsystem(core_subsystem) => Some(core_subsystem.clone().into()),
            _ => None,
        }
    }

    pub fn as_keymap_subsystem(&self) -> Option<KeymapSubsystem> {
        match &self.0 {
            cddl_lib::Request::KeymapSubsystem(keymap_subsystem) => {
                Some(keymap_subsystem.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_behavior_subsystem(&self) -> Option<BehaviorSubsystem> {
        match &self.0 {
            cddl_lib::Request::BehaviorSubsystem(behavior_subsystem) => {
                Some(behavior_subsystem.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cddl_lib::Request> for Request {
    fn from(native: cddl_lib::Request) -> Self {
        Self(native)
    }
}

impl From<Request> for cddl_lib::Request {
    fn from(wasm: Request) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::Request> for Request {
    fn as_ref(&self) -> &cddl_lib::Request {
        &self.0
    }
}

#[wasm_bindgen]
pub enum RequestKind {
    CoreSubsystem,
    KeymapSubsystem,
    BehaviorSubsystem,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnlockRequest(cddl_lib::UnlockRequest);

#[wasm_bindgen]
impl UnlockRequest {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<UnlockRequest, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::UnlockRequest::new())
    }
}

impl From<cddl_lib::UnlockRequest> for UnlockRequest {
    fn from(native: cddl_lib::UnlockRequest) -> Self {
        Self(native)
    }
}

impl From<UnlockRequest> for cddl_lib::UnlockRequest {
    fn from(wasm: UnlockRequest) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::UnlockRequest> for UnlockRequest {
    fn as_ref(&self) -> &cddl_lib::UnlockRequest {
        &self.0
    }
}
