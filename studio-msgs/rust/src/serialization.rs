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
