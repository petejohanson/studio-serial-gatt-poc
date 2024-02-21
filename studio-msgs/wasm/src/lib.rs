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
pub struct BehaviorResponse(cddl_lib::BehaviorResponse);

#[wasm_bindgen]
impl BehaviorResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BehaviorResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn list_all_behaviors_response(&self) -> ListAllBehaviorsResponse {
        self.0.list_all_behaviors_response.clone().into()
    }

    pub fn new(list_all_behaviors_response: &ListAllBehaviorsResponse) -> Self {
        Self(cddl_lib::BehaviorResponse::new(
            list_all_behaviors_response.clone().into(),
        ))
    }
}

impl From<cddl_lib::BehaviorResponse> for BehaviorResponse {
    fn from(native: cddl_lib::BehaviorResponse) -> Self {
        Self(native)
    }
}

impl From<BehaviorResponse> for cddl_lib::BehaviorResponse {
    fn from(wasm: BehaviorResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::BehaviorResponse> for BehaviorResponse {
    fn as_ref(&self) -> &cddl_lib::BehaviorResponse {
        &self.0
    }
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
pub struct BehaviorSummary(cddl_lib::BehaviorSummary);

#[wasm_bindgen]
impl BehaviorSummary {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BehaviorSummary, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    pub fn new(name: String) -> Self {
        Self(cddl_lib::BehaviorSummary::new(name))
    }
}

impl From<cddl_lib::BehaviorSummary> for BehaviorSummary {
    fn from(native: cddl_lib::BehaviorSummary) -> Self {
        Self(native)
    }
}

impl From<BehaviorSummary> for cddl_lib::BehaviorSummary {
    fn from(wasm: BehaviorSummary) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::BehaviorSummary> for BehaviorSummary {
    fn as_ref(&self) -> &cddl_lib::BehaviorSummary {
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
pub struct CoreResponse(cddl_lib::CoreResponse);

#[wasm_bindgen]
impl CoreResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<CoreResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn index_1(&self) -> GetLockStateResponseOrUnlockResponseOrLockResponse {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &GetLockStateResponseOrUnlockResponseOrLockResponse) -> Self {
        Self(cddl_lib::CoreResponse::new(index_1.clone().into()))
    }
}

impl From<cddl_lib::CoreResponse> for CoreResponse {
    fn from(native: cddl_lib::CoreResponse) -> Self {
        Self(native)
    }
}

impl From<CoreResponse> for cddl_lib::CoreResponse {
    fn from(wasm: CoreResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::CoreResponse> for CoreResponse {
    fn as_ref(&self) -> &cddl_lib::CoreResponse {
        &self.0
    }
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
pub struct GetLayersSummaryResponse(cddl_lib::GetLayersSummaryResponse);

#[wasm_bindgen]
impl GetLayersSummaryResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GetLayersSummaryResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn keymap_layer_summary(&self) -> KeymapLayerSummary {
        self.0.keymap_layer_summary.clone().into()
    }

    pub fn new(keymap_layer_summary: &KeymapLayerSummary) -> Self {
        Self(cddl_lib::GetLayersSummaryResponse::new(
            keymap_layer_summary.clone().into(),
        ))
    }
}

impl From<cddl_lib::GetLayersSummaryResponse> for GetLayersSummaryResponse {
    fn from(native: cddl_lib::GetLayersSummaryResponse) -> Self {
        Self(native)
    }
}

impl From<GetLayersSummaryResponse> for cddl_lib::GetLayersSummaryResponse {
    fn from(wasm: GetLayersSummaryResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetLayersSummaryResponse> for GetLayersSummaryResponse {
    fn as_ref(&self) -> &cddl_lib::GetLayersSummaryResponse {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GetLockStateResponse(cddl_lib::GetLockStateResponse);

#[wasm_bindgen]
impl GetLockStateResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GetLockStateResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn locked(&self) -> bool {
        self.0.locked
    }

    pub fn new(locked: bool) -> Self {
        Self(cddl_lib::GetLockStateResponse::new(locked))
    }
}

impl From<cddl_lib::GetLockStateResponse> for GetLockStateResponse {
    fn from(native: cddl_lib::GetLockStateResponse) -> Self {
        Self(native)
    }
}

impl From<GetLockStateResponse> for cddl_lib::GetLockStateResponse {
    fn from(wasm: GetLockStateResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetLockStateResponse> for GetLockStateResponse {
    fn as_ref(&self) -> &cddl_lib::GetLockStateResponse {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GetLockStateResponseOrUnlockResponseOrLockResponse(
    cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse,
);

#[wasm_bindgen]
impl GetLockStateResponseOrUnlockResponseOrLockResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(
        cbor_bytes: &[u8],
    ) -> Result<GetLockStateResponseOrUnlockResponseOrLockResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new_get_lock_state_response(get_lock_state_response: &GetLockStateResponse) -> Self {
        Self(cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::new_get_lock_state_response(get_lock_state_response.clone().into()))
    }

    pub fn new_unlock_response(unlock_response: &UnlockResponse) -> Self {
        Self(
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::new_unlock_response(
                unlock_response.clone().into(),
            ),
        )
    }

    pub fn new_lock_response(lock_response: &LockResponse) -> Self {
        Self(
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::new_lock_response(
                lock_response.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> GetLockStateResponseOrUnlockResponseOrLockResponseKind {
        match &self.0 {
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::GetLockStateResponse(
                _,
            ) => GetLockStateResponseOrUnlockResponseOrLockResponseKind::GetLockStateResponse,
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::UnlockResponse(_) => {
                GetLockStateResponseOrUnlockResponseOrLockResponseKind::UnlockResponse
            }
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::LockResponse(_) => {
                GetLockStateResponseOrUnlockResponseOrLockResponseKind::LockResponse
            }
        }
    }

    pub fn as_get_lock_state_response(&self) -> Option<GetLockStateResponse> {
        match &self.0 {
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::GetLockStateResponse(
                get_lock_state_response,
            ) => Some(get_lock_state_response.clone().into()),
            _ => None,
        }
    }

    pub fn as_unlock_response(&self) -> Option<UnlockResponse> {
        match &self.0 {
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::UnlockResponse(
                unlock_response,
            ) => Some(unlock_response.clone().into()),
            _ => None,
        }
    }

    pub fn as_lock_response(&self) -> Option<LockResponse> {
        match &self.0 {
            cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse::LockResponse(
                lock_response,
            ) => Some(lock_response.clone().into()),
            _ => None,
        }
    }
}

impl From<cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse>
    for GetLockStateResponseOrUnlockResponseOrLockResponse
{
    fn from(native: cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse) -> Self {
        Self(native)
    }
}

impl From<GetLockStateResponseOrUnlockResponseOrLockResponse>
    for cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse
{
    fn from(wasm: GetLockStateResponseOrUnlockResponseOrLockResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse>
    for GetLockStateResponseOrUnlockResponseOrLockResponse
{
    fn as_ref(&self) -> &cddl_lib::GetLockStateResponseOrUnlockResponseOrLockResponse {
        &self.0
    }
}

#[wasm_bindgen]
pub enum GetLockStateResponseOrUnlockResponseOrLockResponseKind {
    GetLockStateResponse,
    UnlockResponse,
    LockResponse,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeymapLayerSummary(cddl_lib::KeymapLayerSummary);

#[wasm_bindgen]
impl KeymapLayerSummary {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeymapLayerSummary, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    pub fn enabled(&self) -> bool {
        self.0.enabled
    }

    pub fn new(name: String, enabled: bool) -> Self {
        Self(cddl_lib::KeymapLayerSummary::new(name, enabled))
    }
}

impl From<cddl_lib::KeymapLayerSummary> for KeymapLayerSummary {
    fn from(native: cddl_lib::KeymapLayerSummary) -> Self {
        Self(native)
    }
}

impl From<KeymapLayerSummary> for cddl_lib::KeymapLayerSummary {
    fn from(wasm: KeymapLayerSummary) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::KeymapLayerSummary> for KeymapLayerSummary {
    fn as_ref(&self) -> &cddl_lib::KeymapLayerSummary {
        &self.0
    }
}

pub type KeymapRequest = GetLayersSummary;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeymapResponse(cddl_lib::KeymapResponse);

#[wasm_bindgen]
impl KeymapResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeymapResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn get_layers_summary_response(&self) -> GetLayersSummaryResponse {
        self.0.get_layers_summary_response.clone().into()
    }

    pub fn new(get_layers_summary_response: &GetLayersSummaryResponse) -> Self {
        Self(cddl_lib::KeymapResponse::new(
            get_layers_summary_response.clone().into(),
        ))
    }
}

impl From<cddl_lib::KeymapResponse> for KeymapResponse {
    fn from(native: cddl_lib::KeymapResponse) -> Self {
        Self(native)
    }
}

impl From<KeymapResponse> for cddl_lib::KeymapResponse {
    fn from(wasm: KeymapResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::KeymapResponse> for KeymapResponse {
    fn as_ref(&self) -> &cddl_lib::KeymapResponse {
        &self.0
    }
}

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
pub struct ListAllBehaviorsResponse(cddl_lib::ListAllBehaviorsResponse);

#[wasm_bindgen]
impl ListAllBehaviorsResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ListAllBehaviorsResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn behavior_summary(&self) -> BehaviorSummary {
        self.0.behavior_summary.clone().into()
    }

    pub fn new(behavior_summary: &BehaviorSummary) -> Self {
        Self(cddl_lib::ListAllBehaviorsResponse::new(
            behavior_summary.clone().into(),
        ))
    }
}

impl From<cddl_lib::ListAllBehaviorsResponse> for ListAllBehaviorsResponse {
    fn from(native: cddl_lib::ListAllBehaviorsResponse) -> Self {
        Self(native)
    }
}

impl From<ListAllBehaviorsResponse> for cddl_lib::ListAllBehaviorsResponse {
    fn from(wasm: ListAllBehaviorsResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::ListAllBehaviorsResponse> for ListAllBehaviorsResponse {
    fn as_ref(&self) -> &cddl_lib::ListAllBehaviorsResponse {
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
pub struct LockResponse(cddl_lib::LockResponse);

#[wasm_bindgen]
impl LockResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<LockResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::LockResponse::new())
    }
}

impl From<cddl_lib::LockResponse> for LockResponse {
    fn from(native: cddl_lib::LockResponse) -> Self {
        Self(native)
    }
}

impl From<LockResponse> for cddl_lib::LockResponse {
    fn from(wasm: LockResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::LockResponse> for LockResponse {
    fn as_ref(&self) -> &cddl_lib::LockResponse {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Notification(cddl_lib::Notification);

#[wasm_bindgen]
impl Notification {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Notification, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn notification_payload(&self) -> NotificationPayload {
        self.0.notification_payload
    }

    pub fn new(notification_payload: NotificationPayload) -> Self {
        Self(cddl_lib::Notification::new(notification_payload))
    }
}

impl From<cddl_lib::Notification> for Notification {
    fn from(native: cddl_lib::Notification) -> Self {
        Self(native)
    }
}

impl From<Notification> for cddl_lib::Notification {
    fn from(wasm: Notification) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::Notification> for Notification {
    fn as_ref(&self) -> &cddl_lib::Notification {
        &self.0
    }
}

pub type NotificationPayload = u64;

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
pub struct RequestResponse(cddl_lib::RequestResponse);

#[wasm_bindgen]
impl RequestResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<RequestResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn response_payload(&self) -> ResponsePayload {
        self.0.response_payload.clone().into()
    }

    pub fn new(response_payload: &ResponsePayload) -> Self {
        Self(cddl_lib::RequestResponse::new(
            response_payload.clone().into(),
        ))
    }
}

impl From<cddl_lib::RequestResponse> for RequestResponse {
    fn from(native: cddl_lib::RequestResponse) -> Self {
        Self(native)
    }
}

impl From<RequestResponse> for cddl_lib::RequestResponse {
    fn from(wasm: RequestResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::RequestResponse> for RequestResponse {
    fn as_ref(&self) -> &cddl_lib::RequestResponse {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Response(cddl_lib::Response);

#[wasm_bindgen]
impl Response {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Response, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new_notification(notification: &Notification) -> Self {
        Self(cddl_lib::Response::new_notification(
            notification.clone().into(),
        ))
    }

    pub fn new_request_response(request_response: &RequestResponse) -> Self {
        Self(cddl_lib::Response::new_request_response(
            request_response.clone().into(),
        ))
    }

    pub fn kind(&self) -> ResponseKind {
        match &self.0 {
            cddl_lib::Response::Notification(_) => ResponseKind::Notification,
            cddl_lib::Response::RequestResponse(_) => ResponseKind::RequestResponse,
        }
    }

    pub fn as_notification(&self) -> Option<Notification> {
        match &self.0 {
            cddl_lib::Response::Notification(notification) => Some(notification.clone().into()),
            _ => None,
        }
    }

    pub fn as_request_response(&self) -> Option<RequestResponse> {
        match &self.0 {
            cddl_lib::Response::RequestResponse(request_response) => {
                Some(request_response.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cddl_lib::Response> for Response {
    fn from(native: cddl_lib::Response) -> Self {
        Self(native)
    }
}

impl From<Response> for cddl_lib::Response {
    fn from(wasm: Response) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::Response> for Response {
    fn as_ref(&self) -> &cddl_lib::Response {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ResponseKind {
    Notification,
    RequestResponse,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ResponsePayload(cddl_lib::ResponsePayload);

#[wasm_bindgen]
impl ResponsePayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ResponsePayload, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new_core_response(core_response: &CoreResponse) -> Self {
        Self(cddl_lib::ResponsePayload::new_core_response(
            core_response.clone().into(),
        ))
    }

    pub fn new_keymap_response(keymap_response: &KeymapResponse) -> Self {
        Self(cddl_lib::ResponsePayload::new_keymap_response(
            keymap_response.clone().into(),
        ))
    }

    pub fn new_behavior_response(behavior_response: &BehaviorResponse) -> Self {
        Self(cddl_lib::ResponsePayload::new_behavior_response(
            behavior_response.clone().into(),
        ))
    }

    pub fn kind(&self) -> ResponsePayloadKind {
        match &self.0 {
            cddl_lib::ResponsePayload::CoreResponse(_) => ResponsePayloadKind::CoreResponse,
            cddl_lib::ResponsePayload::KeymapResponse(_) => ResponsePayloadKind::KeymapResponse,
            cddl_lib::ResponsePayload::BehaviorResponse(_) => ResponsePayloadKind::BehaviorResponse,
        }
    }

    pub fn as_core_response(&self) -> Option<CoreResponse> {
        match &self.0 {
            cddl_lib::ResponsePayload::CoreResponse(core_response) => {
                Some(core_response.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_keymap_response(&self) -> Option<KeymapResponse> {
        match &self.0 {
            cddl_lib::ResponsePayload::KeymapResponse(keymap_response) => {
                Some(keymap_response.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_behavior_response(&self) -> Option<BehaviorResponse> {
        match &self.0 {
            cddl_lib::ResponsePayload::BehaviorResponse(behavior_response) => {
                Some(behavior_response.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cddl_lib::ResponsePayload> for ResponsePayload {
    fn from(native: cddl_lib::ResponsePayload) -> Self {
        Self(native)
    }
}

impl From<ResponsePayload> for cddl_lib::ResponsePayload {
    fn from(wasm: ResponsePayload) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::ResponsePayload> for ResponsePayload {
    fn as_ref(&self) -> &cddl_lib::ResponsePayload {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ResponsePayloadKind {
    CoreResponse,
    KeymapResponse,
    BehaviorResponse,
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnlockResponse(cddl_lib::UnlockResponse);

#[wasm_bindgen]
impl UnlockResponse {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cddl_lib::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<UnlockResponse, JsError> {
        cddl_lib::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsError::new(&format!("from_bytes: {}", e)))
    }

    pub fn new() -> Self {
        Self(cddl_lib::UnlockResponse::new())
    }
}

impl From<cddl_lib::UnlockResponse> for UnlockResponse {
    fn from(native: cddl_lib::UnlockResponse) -> Self {
        Self(native)
    }
}

impl From<UnlockResponse> for cddl_lib::UnlockResponse {
    fn from(wasm: UnlockResponse) -> Self {
        wasm.0
    }
}

impl AsRef<cddl_lib::UnlockResponse> for UnlockResponse {
    fn as_ref(&self) -> &cddl_lib::UnlockResponse {
        &self.0
    }
}
