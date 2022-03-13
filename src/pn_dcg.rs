use crate::profinet::FrameId;
use anyhow::bail;
use pnet::packet::ethernet::EtherType;
// use pnet::packet::PacketSize;
use crate::comm::to_u16;
use pnet::util::MacAddr;
use pnet_macros::packet;
use pnet_macros_support::packet::PrimitiveValues;
use pnet_macros_support::types::{u16be, u32be};

// #[packet]
// pub struct PnDcg {
//     #[construct_with(u8, u8, u8, u8, u8, u8)]
//     pub destination: MacAddr,
//     #[construct_with(u8, u8, u8, u8, u8, u8)]
//     pub source: MacAddr,
//     #[construct_with(u16)]
//     pub ethertype: EtherType,
//     #[construct_with(u8, u8)]
//     pub frame_id: DoubleU8s,
//     pub service_id: u8,
//     pub service_type: u8,
//     pub xid: u32be,
//     #[construct_with(u8, u8)]
//     pub reserved: DoubleU8s,
//     pub dcp_len: u16be,
//     #[payload]
//     #[length = "dcp_len"]
//     pub payload: Vec<u8>,
// }

fn check(dcf: &PnDcgPacket) {
    dcf.get_reserved();
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd, Debug)]
pub struct DoubleU8s(pub u8, pub u8);

impl DoubleU8s {
    pub fn new(a: u8, b: u8) -> Self {
        Self(a, b)
    }

    pub fn to_u8s(&self) -> [u8; 2] {
        [self.0, self.1]
    }
}
impl PrimitiveValues for DoubleU8s {
    type T = (u8, u8);

    fn to_primitive_values(&self) -> Self::T {
        (self.0, self.1)
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PnDcgTy {
    HELLO_REQ = 0,    // 0xfe, 0xfc, 0x06, 0x00
    HELLO_RESP_SUC,   // 0xfe, 0xfc, 0x06, 0x01
    HELLO_RESP_UNSUP, // 0xfe, 0xfc, 0x06, 0x05
    IDENT_REQ,        // 0xfe, 0xfe, 0x05, 0x00
    IDENT_RESP_SUC,   // 0xfe, 0xff, 0x05, 0x01
    IDENT_RESP_UNSUP, // 0xfe, 0xff, 0x05, 0x05
    GET_REQ,          // 0xfe, 0xfd, 0x03, 0x00
    GET_RESP_SUC,     // 0xfe, 0xfd, 0x03, 0x01
    GET_RESP_UNSUP,   // 0xfe, 0xfd, 0x03, 0x05
    SET_REQ,          // 0xfe, 0xfd, 0x04, 0x00
    SET_RESP_SUC,     // 0xfe, 0xfd, 0x04, 0x01
    SET_RESP_UNSUP,   // 0xfe, 0xfd, 0x04, 0x05
    UNSUPPORT = 12,
}
impl PnDcgTy {
    pub fn is_unsupport(&self) -> bool {
        if self as u23 == 12 {
            true
        } else {
            false
        }
    }
}
impl From<[u8; 4]> for PnDcgTy {
    fn from(data: [u8; 4]) -> Self {
        match data {
            [0xfe, 0xfc, 0x06, 0x00] => Self::HELLO_REQ,
            [0xfe, 0xfc, 0x06, 0x01] => Self::HELLO_RESP_SUC,
            [0xfe, 0xfc, 0x06, 0x05] => Self::HELLO_RESP_UNSUP,
            [0xfe, 0xfe, 0x05, 0x00] => Self::IDENT_REQ,
            [0xfe, 0xff, 0x05, 0x01] => Self::IDENT_RESP_SUC,
            [0xfe, 0xff, 0x05, 0x05] => Self::IDENT_RESP_UNSUP,
            [0xfe, 0xfd, 0x03, 0x00] => Self::GET_REQ,
            [0xfe, 0xfd, 0x03, 0x01] => Self::GET_RESP_SUC,
            [0xfe, 0xfd, 0x03, 0x05] => Self::GET_RESP_UNSUP,
            [0xfe, 0xfd, 0x04, 0x00] => Self::SET_REQ,
            [0xfe, 0xfd, 0x04, 0x01] => Self::SET_RESP_SUC,
            [0xfe, 0xfd, 0x04, 0x05] => Self::SET_RESP_UNSUP,
            _ => Self::UNSUPPORT,
        }
    }
}
pub struct PnDcg {
    data: Vec<u8>,
    ty: PnDcgTy,
    xid: [u8; 4],
    // reserved: (u8, u8),
    payload_len: usize,
}

impl TryFrom<&[u8]> for PnDcg {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some(payload_len) = value
            .get(25)
            .and_then(|x| Some(to_u16(value[24], *x) as usize))
        {
            let ty = PnDcgTy::from([value[14], value[15], value[16], value[17]]);
            if ty.is_unsupport() {
                bail!("todo");
            }
            let xid = [value[18], value[19], value[20], value[21]];
            if payload_len + 16 != value.len() {
                bail!("todo");
            }
            Ok(Self {
                data: value.to_vec(),
                ty,
                xid,
                payload_len,
            })
        }
        bail!("长度不足，非PnDcg包");
    }
}
