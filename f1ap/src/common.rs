// Autogenerated from F1AP-CommonDataTypes.asn
#![allow(clippy::all)]
use asn1_per::{aper::*, *};

// Criticality
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Criticality {
    Reject,
    Ignore,
    Notify,
}

impl Criticality {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl PerCodec for Criticality {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Criticality::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("Criticality");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("Criticality");
            e
        })
    }
}
// Presence
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Presence {
    Optional,
    Conditional,
    Mandatory,
}

impl Presence {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl PerCodec for Presence {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Presence::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("Presence");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("Presence");
            e
        })
    }
}
// PrivateIeId
#[derive(Clone, Debug)]
pub enum PrivateIeId {
    Local(u16),
    Global(Vec<u8>),
}

impl PrivateIeId {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"));
        }
        match idx {
            0 => Ok(Self::Local(
                decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
            )),
            1 => Ok(Self::Global(decode::decode_octetstring(
                data, None, None, false,
            )?)),
            _ => Err(PerCodecError::new("Unknown choice idx")),
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::Local(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                encode::encode_integer(data, Some(0), Some(65535), false, *x as i128, false)
            }
            Self::Global(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 1, false)?;
                encode::encode_octetstring(data, None, None, false, &x, false)
            }
        }
    }
}

impl PerCodec for PrivateIeId {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        PrivateIeId::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("PrivateIeId");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("PrivateIeId");
            e
        })
    }
}
// ProcedureCode
#[derive(Clone, Copy, Debug)]
pub struct ProcedureCode(pub u8);

impl ProcedureCode {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(
            decode::decode_integer(data, Some(0), Some(255), false)?.0 as u8,
        ))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(0), Some(255), false, self.0 as i128, false)
    }
}

impl PerCodec for ProcedureCode {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        ProcedureCode::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("ProcedureCode");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("ProcedureCode");
            e
        })
    }
}
// ProtocolExtensionId
#[derive(Clone, Copy, Debug)]
pub struct ProtocolExtensionId(pub u16);

impl ProtocolExtensionId {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(
            decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
        ))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(0), Some(65535), false, self.0 as i128, false)
    }
}

impl PerCodec for ProtocolExtensionId {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        ProtocolExtensionId::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("ProtocolExtensionId");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("ProtocolExtensionId");
            e
        })
    }
}
// ProtocolIeId
#[derive(Clone, Copy, Debug)]
pub struct ProtocolIeId(pub u16);

impl ProtocolIeId {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(
            decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
        ))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(0), Some(65535), false, self.0 as i128, false)
    }
}

impl PerCodec for ProtocolIeId {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        ProtocolIeId::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("ProtocolIeId");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("ProtocolIeId");
            e
        })
    }
}
// TriggeringMessage
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum TriggeringMessage {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}

impl TriggeringMessage {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl PerCodec for TriggeringMessage {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        TriggeringMessage::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("TriggeringMessage");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("TriggeringMessage");
            e
        })
    }
}
