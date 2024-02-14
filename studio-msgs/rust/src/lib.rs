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
pub struct BehaviorSubsystem {
    pub behavior_request: BehaviorRequest,
}

impl BehaviorSubsystem {
    pub fn new(behavior_request: BehaviorRequest) -> Self {
        Self { behavior_request }
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

pub type KeymapRequest = GetLayersSummary;

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
