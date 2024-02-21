// same as cbor_event::de::Deserialize but with our DeserializeError
pub trait Deserialize {
    fn from_cbor_bytes(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Self::deserialize(&mut raw)
    }

    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

impl<T: cbor_event::de::Deserialize> Deserialize for T {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<T, DeserializeError> {
        T::deserialize(raw).map_err(DeserializeError::from)
    }
}
pub struct CBORReadLen {
    deser_len: cbor_event::Len,
    read: u64,
}

impl CBORReadLen {
    pub fn new(len: cbor_event::Len) -> Self {
        Self {
            deser_len: len,
            read: 0,
        }
    }

    pub fn read(&self) -> u64 {
        self.read
    }

    // Marks {n} values as being read, and if we go past the available definite length
    // given by the CBOR, we return an error.
    pub fn read_elems(&mut self, count: usize) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::Len::Len(n) => {
                self.read += count as u64;
                if self.read > n {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, None))
                } else {
                    Ok(())
                }
            }
            cbor_event::Len::Indefinite => Ok(()),
        }
    }

    pub fn finish(&self) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::Len::Len(n) => {
                if self.read == n {
                    Ok(())
                } else {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, Some(self.read)))
                }
            }
            cbor_event::Len::Indefinite => Ok(()),
        }
    }
}

pub trait DeserializeEmbeddedGroup {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        read_len: &mut CBORReadLen,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}
pub trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

pub trait ToCBORBytes {
    fn to_cbor_bytes(&self) -> Vec<u8>;
}

impl<T: cbor_event::se::Serialize> ToCBORBytes for T {
    fn to_cbor_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }
}

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use crate::error::*;
use cbor_event::de::Deserializer;
use cbor_event::se::{Serialize, Serializer};
use std::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for BehaviorRequest {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            BehaviorRequest::ListAllBehaviors(list_all_behaviors) => {
                list_all_behaviors.serialize(serializer)
            }
            BehaviorRequest::GetBehaviorDetails(get_behavior_details) => {
                get_behavior_details.serialize(serializer)
            }
        }
    }
}

impl Deserialize for BehaviorRequest {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ListAllBehaviors::deserialize(raw);
            match deser_variant {
                Ok(list_all_behaviors) => return Ok(Self::ListAllBehaviors(list_all_behaviors)),
                Err(e) => {
                    errs.push(e.annotate("ListAllBehaviors"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = GetBehaviorDetails::deserialize(raw);
            match deser_variant {
                Ok(get_behavior_details) => {
                    return Ok(Self::GetBehaviorDetails(get_behavior_details))
                }
                Err(e) => {
                    errs.push(e.annotate("GetBehaviorDetails"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "BehaviorRequest",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("BehaviorRequest"))
    }
}

impl cbor_event::se::Serialize for BehaviorResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(2u64)?;
        self.list_all_behaviors_response.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for BehaviorResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let list_all_behaviors_response = ListAllBehaviorsResponse::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("list_all_behaviors_response"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BehaviorResponse {
                list_all_behaviors_response,
            })
        })()
        .map_err(|e| e.annotate("BehaviorResponse"))
    }
}

impl cbor_event::se::Serialize for BehaviorSubsystem {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(2u64)?;
        self.behavior_request.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for BehaviorSubsystem {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let behavior_request = BehaviorRequest::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("behavior_request"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BehaviorSubsystem { behavior_request })
        })()
        .map_err(|e| e.annotate("BehaviorSubsystem"))
    }
}

impl cbor_event::se::Serialize for BehaviorSummary {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_text(&self.name)?;
        Ok(serializer)
    }
}

impl Deserialize for BehaviorSummary {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let name =
                Ok(raw.text()? as String).map_err(|e: DeserializeError| e.annotate("name"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BehaviorSummary { name })
        })()
        .map_err(|e| e.annotate("BehaviorSummary"))
    }
}

impl cbor_event::se::Serialize for CoreRequest {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            CoreRequest::GetLockState(get_lock_state) => get_lock_state.serialize(serializer),
            CoreRequest::UnlockRequest(unlock_request) => unlock_request.serialize(serializer),
            CoreRequest::LockRequest(lock_request) => lock_request.serialize(serializer),
        }
    }
}

impl Deserialize for CoreRequest {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = GetLockState::deserialize(raw);
            match deser_variant {
                Ok(get_lock_state) => return Ok(Self::GetLockState(get_lock_state)),
                Err(e) => {
                    errs.push(e.annotate("GetLockState"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = UnlockRequest::deserialize(raw);
            match deser_variant {
                Ok(unlock_request) => return Ok(Self::UnlockRequest(unlock_request)),
                Err(e) => {
                    errs.push(e.annotate("UnlockRequest"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = LockRequest::deserialize(raw);
            match deser_variant {
                Ok(lock_request) => return Ok(Self::LockRequest(lock_request)),
                Err(e) => {
                    errs.push(e.annotate("LockRequest"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "CoreRequest",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("CoreRequest"))
    }
}

impl cbor_event::se::Serialize for CoreResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        self.index_1.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CoreResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let index_1 = GetLockStateResponseOrUnlockResponseOrLockResponse::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("index_1"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(CoreResponse { index_1 })
        })()
        .map_err(|e| e.annotate("CoreResponse"))
    }
}

impl cbor_event::se::Serialize for CoreSubsystem {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        self.core_request.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CoreSubsystem {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let core_request = CoreRequest::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("core_request"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(CoreSubsystem { core_request })
        })()
        .map_err(|e| e.annotate("CoreSubsystem"))
    }
}

impl cbor_event::se::Serialize for GetBehaviorDetails {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(1u64)?;
        self.get_behavior_details_payload.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for GetBehaviorDetails {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let get_behavior_details_payload = GetBehaviorDetailsPayload::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("get_behavior_details_payload"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GetBehaviorDetails {
                get_behavior_details_payload,
            })
        })()
        .map_err(|e| e.annotate("GetBehaviorDetails"))
    }
}

impl cbor_event::se::Serialize for GetBehaviorDetailsPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(self.behavior_id)?;
        Ok(serializer)
    }
}

impl Deserialize for GetBehaviorDetailsPayload {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let behavior_id = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("behavior_id"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GetBehaviorDetailsPayload { behavior_id })
        })()
        .map_err(|e| e.annotate("GetBehaviorDetailsPayload"))
    }
}

impl cbor_event::se::Serialize for GetLayersSummary {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(0u64)?;
        Ok(serializer)
    }
}

impl Deserialize for GetLayersSummary {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GetLayersSummary {})
        })()
        .map_err(|e| e.annotate("GetLayersSummary"))
    }
}

impl cbor_event::se::Serialize for GetLayersSummaryResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        self.keymap_layer_summary.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for GetLayersSummaryResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let keymap_layer_summary = KeymapLayerSummary::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("keymap_layer_summary"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GetLayersSummaryResponse {
                keymap_layer_summary,
            })
        })()
        .map_err(|e| e.annotate("GetLayersSummaryResponse"))
    }
}

impl cbor_event::se::Serialize for GetLockState {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(0u64)?;
        Ok(serializer)
    }
}

impl Deserialize for GetLockState {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GetLockState {})
        })()
        .map_err(|e| e.annotate("GetLockState"))
    }
}

impl cbor_event::se::Serialize for GetLockStateResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_special(cbor_event::Special::Bool(self.locked))?;
        Ok(serializer)
    }
}

impl Deserialize for GetLockStateResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let locked = raw
                .bool()
                .map_err(Into::into)
                .map_err(|e: DeserializeError| e.annotate("locked"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GetLockStateResponse { locked })
        })()
        .map_err(|e| e.annotate("GetLockStateResponse"))
    }
}

impl cbor_event::se::Serialize for GetLockStateResponseOrUnlockResponseOrLockResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            GetLockStateResponseOrUnlockResponseOrLockResponse::GetLockStateResponse(
                get_lock_state_response,
            ) => get_lock_state_response.serialize(serializer),
            GetLockStateResponseOrUnlockResponseOrLockResponse::UnlockResponse(unlock_response) => {
                unlock_response.serialize(serializer)
            }
            GetLockStateResponseOrUnlockResponseOrLockResponse::LockResponse(lock_response) => {
                lock_response.serialize(serializer)
            }
        }
    }
}

impl Deserialize for GetLockStateResponseOrUnlockResponseOrLockResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = GetLockStateResponse::deserialize(raw);
            match deser_variant {
                Ok(get_lock_state_response) => {
                    return Ok(Self::GetLockStateResponse(get_lock_state_response))
                }
                Err(e) => {
                    errs.push(e.annotate("GetLockStateResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = UnlockResponse::deserialize(raw);
            match deser_variant {
                Ok(unlock_response) => return Ok(Self::UnlockResponse(unlock_response)),
                Err(e) => {
                    errs.push(e.annotate("UnlockResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = LockResponse::deserialize(raw);
            match deser_variant {
                Ok(lock_response) => return Ok(Self::LockResponse(lock_response)),
                Err(e) => {
                    errs.push(e.annotate("LockResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "GetLockStateResponseOrUnlockResponseOrLockResponse",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("GetLockStateResponseOrUnlockResponseOrLockResponse"))
    }
}

impl cbor_event::se::Serialize for KeymapLayerSummary {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_text(&self.name)?;
        serializer.write_special(cbor_event::Special::Bool(self.enabled))?;
        Ok(serializer)
    }
}

impl Deserialize for KeymapLayerSummary {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let name =
                Ok(raw.text()? as String).map_err(|e: DeserializeError| e.annotate("name"))?;
            let enabled = raw
                .bool()
                .map_err(Into::into)
                .map_err(|e: DeserializeError| e.annotate("enabled"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(KeymapLayerSummary { name, enabled })
        })()
        .map_err(|e| e.annotate("KeymapLayerSummary"))
    }
}

impl cbor_event::se::Serialize for KeymapResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(1u64)?;
        self.get_layers_summary_response.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for KeymapResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let get_layers_summary_response = GetLayersSummaryResponse::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("get_layers_summary_response"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(KeymapResponse {
                get_layers_summary_response,
            })
        })()
        .map_err(|e| e.annotate("KeymapResponse"))
    }
}

impl cbor_event::se::Serialize for KeymapSubsystem {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(1u64)?;
        self.keymap_request.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for KeymapSubsystem {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let keymap_request = GetLayersSummary::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("keymap_request"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(KeymapSubsystem { keymap_request })
        })()
        .map_err(|e| e.annotate("KeymapSubsystem"))
    }
}

impl cbor_event::se::Serialize for ListAllBehaviors {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(0u64)?;
        Ok(serializer)
    }
}

impl Deserialize for ListAllBehaviors {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ListAllBehaviors {})
        })()
        .map_err(|e| e.annotate("ListAllBehaviors"))
    }
}

impl cbor_event::se::Serialize for ListAllBehaviorsResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(1u64)?;
        self.behavior_summary.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ListAllBehaviorsResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let behavior_summary = BehaviorSummary::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("behavior_summary"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ListAllBehaviorsResponse { behavior_summary })
        })()
        .map_err(|e| e.annotate("ListAllBehaviorsResponse"))
    }
}

impl cbor_event::se::Serialize for LockRequest {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(2u64)?;
        Ok(serializer)
    }
}

impl Deserialize for LockRequest {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(LockRequest {})
        })()
        .map_err(|e| e.annotate("LockRequest"))
    }
}

impl cbor_event::se::Serialize for LockResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(2u64)?;
        Ok(serializer)
    }
}

impl Deserialize for LockResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(LockResponse {})
        })()
        .map_err(|e| e.annotate("LockResponse"))
    }
}

impl cbor_event::se::Serialize for Notification {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_unsigned_integer(self.notification_payload)?;
        Ok(serializer)
    }
}

impl Deserialize for Notification {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let notification_payload = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("notification_payload"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Notification {
                notification_payload,
            })
        })()
        .map_err(|e| e.annotate("Notification"))
    }
}

impl cbor_event::se::Serialize for Request {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Request::CoreSubsystem(core_subsystem) => core_subsystem.serialize(serializer),
            Request::KeymapSubsystem(keymap_subsystem) => keymap_subsystem.serialize(serializer),
            Request::BehaviorSubsystem(behavior_subsystem) => {
                behavior_subsystem.serialize(serializer)
            }
        }
    }
}

impl Deserialize for Request {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = CoreSubsystem::deserialize(raw);
            match deser_variant {
                Ok(core_subsystem) => return Ok(Self::CoreSubsystem(core_subsystem)),
                Err(e) => {
                    errs.push(e.annotate("CoreSubsystem"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = KeymapSubsystem::deserialize(raw);
            match deser_variant {
                Ok(keymap_subsystem) => return Ok(Self::KeymapSubsystem(keymap_subsystem)),
                Err(e) => {
                    errs.push(e.annotate("KeymapSubsystem"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = BehaviorSubsystem::deserialize(raw);
            match deser_variant {
                Ok(behavior_subsystem) => return Ok(Self::BehaviorSubsystem(behavior_subsystem)),
                Err(e) => {
                    errs.push(e.annotate("BehaviorSubsystem"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Request",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Request"))
    }
}

impl cbor_event::se::Serialize for RequestResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(1u64)?;
        self.response_payload.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for RequestResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let response_payload = ResponsePayload::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("response_payload"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(RequestResponse { response_payload })
        })()
        .map_err(|e| e.annotate("RequestResponse"))
    }
}

impl cbor_event::se::Serialize for Response {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Response::Notification(notification) => notification.serialize(serializer),
            Response::RequestResponse(request_response) => request_response.serialize(serializer),
        }
    }
}

impl Deserialize for Response {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = Notification::deserialize(raw);
            match deser_variant {
                Ok(notification) => return Ok(Self::Notification(notification)),
                Err(e) => {
                    errs.push(e.annotate("Notification"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = RequestResponse::deserialize(raw);
            match deser_variant {
                Ok(request_response) => return Ok(Self::RequestResponse(request_response)),
                Err(e) => {
                    errs.push(e.annotate("RequestResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Response",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Response"))
    }
}

impl cbor_event::se::Serialize for ResponsePayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ResponsePayload::CoreResponse(core_response) => core_response.serialize(serializer),
            ResponsePayload::KeymapResponse(keymap_response) => {
                keymap_response.serialize(serializer)
            }
            ResponsePayload::BehaviorResponse(behavior_response) => {
                behavior_response.serialize(serializer)
            }
        }
    }
}

impl Deserialize for ResponsePayload {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = CoreResponse::deserialize(raw);
            match deser_variant {
                Ok(core_response) => return Ok(Self::CoreResponse(core_response)),
                Err(e) => {
                    errs.push(e.annotate("CoreResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = KeymapResponse::deserialize(raw);
            match deser_variant {
                Ok(keymap_response) => return Ok(Self::KeymapResponse(keymap_response)),
                Err(e) => {
                    errs.push(e.annotate("KeymapResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = BehaviorResponse::deserialize(raw);
            match deser_variant {
                Ok(behavior_response) => return Ok(Self::BehaviorResponse(behavior_response)),
                Err(e) => {
                    errs.push(e.annotate("BehaviorResponse"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "ResponsePayload",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ResponsePayload"))
    }
}

impl cbor_event::se::Serialize for UnlockRequest {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(1u64)?;
        Ok(serializer)
    }
}

impl Deserialize for UnlockRequest {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(UnlockRequest {})
        })()
        .map_err(|e| e.annotate("UnlockRequest"))
    }
}

impl cbor_event::se::Serialize for UnlockResponse {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(1u64)?;
        Ok(serializer)
    }
}

impl Deserialize for UnlockResponse {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(UnlockResponse {})
        })()
        .map_err(|e| e.annotate("UnlockResponse"))
    }
}
