pub mod extend_trait;
pub mod get_req;
pub mod get_resp;
pub mod ident_req;
pub mod ident_resp;
pub mod set_req;
pub mod set_resp;

use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use pnet::packet::ethernet::EtherType;
// use pnet::packet::PacketSize;
use crate::comm::{slice_copy_to_vec, to_u16, u16_to_u8s, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use pnet::util::{MacAddr, Octets};
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

// fn check(dcf: &PnDcgPacket) {
//     dcf.get_reserved();
// }

// #[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd, Debug)]
// pub struct DoubleU8s(pub u8, pub u8);
//
// impl DoubleU8s {
//     pub fn new(a: u8, b: u8) -> Self {
//         Self(a, b)
//     }
//
//     pub fn to_u8s(&self) -> [u8; 2] {
//         [self.0, self.1]
//     }
// }
// impl PrimitiveValues for DoubleU8s {
//     type T = (u8, u8);
//
//     fn to_primitive_values(&self) -> Self::T {
//         (self.0, self.1)
//     }
// }
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PnDcpTy {
    HelloReq,       // 0xfe, 0xfc, 0x06, 0x00
    HelloRespSuc,   // 0xfe, 0xfc, 0x06, 0x01
    HelloRespUnsup, // 0xfe, 0xfc, 0x06, 0x05
    IdentReq,       // 0xfe, 0xfe, 0x05, 0x00
    IdentRespSuc,   // 0xfe, 0xff, 0x05, 0x01
    // IdentRespUnsup, // 0xfe, 0xff, 0x05, 0x05
    GetReq,       // 0xfe, 0xfd, 0x03, 0x00
    GetRespSuc,   // 0xfe, 0xfd, 0x03, 0x01
    GetRespUnsup, // 0xfe, 0xfd, 0x03, 0x05
    SetReq,       // 0xfe, 0xfd, 0x04, 0x00
    SetRespSuc,   // 0xfe, 0xfd, 0x04, 0x01
    SetRespUnsup, // 0xfe, 0xfd, 0x04, 0x05
}
impl PnDcpTy {
    pub fn to_u8_array(&self) -> [u8; 4] {
        match self {
            Self::HelloReq => [0xfe, 0xfc, 0x06, 0x00],
            Self::HelloRespSuc => [0xfe, 0xfc, 0x06, 0x01],
            Self::HelloRespUnsup => [0xfe, 0xfc, 0x06, 0x05],
            Self::IdentReq => [0xfe, 0xfe, 0x05, 0x00],
            Self::IdentRespSuc => [0xfe, 0xff, 0x05, 0x01],
            // Self::IdentRespUnsup => [0xfe, 0xff, 0x05, 0x05],
            Self::GetReq => [0xfe, 0xfd, 0x03, 0x00],
            Self::GetRespSuc => [0xfe, 0xfd, 0x03, 0x01],
            Self::GetRespUnsup => [0xfe, 0xfd, 0x03, 0x05],
            Self::SetReq => [0xfe, 0xfd, 0x04, 0x00],
            Self::SetRespSuc => [0xfe, 0xfd, 0x04, 0x01],
            Self::SetRespUnsup => [0xfe, 0xfd, 0x04, 0x05],
        }
    }
}
impl TryFrom<[u8; 4]> for PnDcpTy {
    type Error = anyhow::Error;
    fn try_from(value: [u8; 4]) -> std::result::Result<Self, Self::Error> {
        match value {
            [0xfe, 0xfc, 0x06, 0x00] => Ok(Self::HelloReq),
            [0xfe, 0xfc, 0x06, 0x01] => Ok(Self::HelloRespSuc),
            [0xfe, 0xfc, 0x06, 0x05] => Ok(Self::HelloRespUnsup),
            [0xfe, 0xfe, 0x05, 0x00] => Ok(Self::IdentReq),
            [0xfe, 0xff, 0x05, 0x01] => Ok(Self::IdentRespSuc),
            // [0xfe, 0xff, 0x05, 0x05] => Ok(Self::IdentRespUnsup),
            [0xfe, 0xfd, 0x03, 0x00] => Ok(Self::GetReq),
            [0xfe, 0xfd, 0x03, 0x01] => Ok(Self::GetRespSuc),
            [0xfe, 0xfd, 0x03, 0x05] => Ok(Self::GetRespUnsup),
            [0xfe, 0xfd, 0x04, 0x00] => Ok(Self::SetReq),
            [0xfe, 0xfd, 0x04, 0x01] => Ok(Self::SetRespSuc),
            [0xfe, 0xfd, 0x04, 0x05] => Ok(Self::SetRespUnsup),
            _ => bail!("todo"),
        }
    }
}
pub struct PnDcgBuilder {
    des: Result<MacAddr>,
    src: Result<MacAddr>,
    ty: PnDcpTy,
    reserved: [u8; 2],
    xid: Result<[u8; 4]>,
    payload: Result<Vec<u8>>,
}
impl PnDcgBuilder {
    pub fn new(ty: PnDcpTy) -> Self {
        Self {
            ty,
            des: Err(anyhow!("todo")),
            src: Err(anyhow!("todo")),
            xid: Err(anyhow!("todo")),
            payload: Err(anyhow!("todo")),
            reserved: [0, 0],
        }
    }
    pub fn build(self) -> Result<Vec<u8>> {
        let Self {
            des,
            src,
            xid,
            payload,
            ty,
            reserved,
        } = self;
        let des = des?;
        let src = src?;
        let xid = xid?;
        let payload = payload?;
        let mut data = Vec::with_capacity(payload.len() + 26);
        slice_copy_to_vec(&mut data, &des.octets());
        slice_copy_to_vec(&mut data, &src.octets());
        slice_copy_to_vec(&mut data, &PROFINET_ETHER_TYPE.0.octets());

        slice_copy_to_vec(&mut data, &ty.to_u8_array());
        slice_copy_to_vec(&mut data, &xid);
        slice_copy_to_vec(&mut data, &reserved);
        slice_copy_to_vec(&mut data, &u16_to_u8s(payload.len() as u16));
        slice_copy_to_vec(&mut data, payload.as_slice());
        Ok(data)
    }
    pub fn set_response_delay(mut self, response_delay: u16) -> Self {
        self.reserved = u16_to_u8s(response_delay);
        self
    }
    pub fn set_reserved(mut self, reserved: [u8; 2]) -> Self {
        self.reserved = reserved;
        self
    }
    pub fn set_des(mut self, des: MacAddr) -> Self {
        self.des = Ok(des);
        self
    }
    pub fn set_des_array(mut self, a: [u8; 6]) -> Self {
        self.des = Ok(MacAddr::new(a[0], a[1], a[2], a[3], a[4], a[5]));
        self
    }
    pub fn set_src_array(mut self, a: [u8; 6]) -> Self {
        self.src = Ok(MacAddr::new(a[0], a[1], a[2], a[3], a[4], a[5]));
        self
    }
    pub fn set_src(mut self, src: MacAddr) -> Self {
        self.src = Ok(src);
        self
    }
    pub fn set_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = Ok(payload);
        self
    }
    pub fn set_xid(mut self, xid: [u8; 4]) -> Self {
        self.xid = Ok(xid);
        self
    }
}
pub struct PnDcg {
    // data: Bytes,
    pub head: DcgHead,
    pub blocks: BytesWrap,
}

impl TryFrom<&[u8]> for PnDcg {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let head = DcgHead::try_from(value)?;
        if let Some(blocks_data) = value.get(26..) {
            let blocks: BytesWrap = blocks_data.to_vec().into();
            return Ok(Self { head, blocks });
        }
        bail!("长度不足，非PnDcg包");
    }
}

#[derive(Debug)]
pub struct DcgHead {
    pub destination: MacAddr,
    pub source: MacAddr,
    pub ty: PnDcpTy,
    pub xid: [u8; 4],
    pub reserved_or_delay: [u8; 2],
    pub payload_len: usize,
}

impl DcgHead {
    pub fn append_data(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(self.destination.octets().as_slice());
        data.extend_from_slice(self.source.octets().as_slice());
        data.extend_from_slice(PROFINET_ETHER_TYPE.0.to_be_bytes().as_slice());
        data.extend_from_slice(self.ty.to_u8_array().as_slice());
        data.extend_from_slice(self.xid.as_slice());
        data.extend_from_slice(self.reserved_or_delay.as_slice());
        data.extend_from_slice((self.payload_len as u16).to_be_bytes().as_slice());
    }
    pub fn new(destination: MacAddr, source: MacAddr, ty: PnDcpTy) -> Self {
        Self {
            destination,
            source,
            ty,
            xid: [0u8; 4],
            reserved_or_delay: [0u8; 2],
            payload_len: 0,
        }
    }
    pub fn set_xid(&mut self, xid: [u8; 4]) {
        self.xid = xid;
    }
    pub fn set_reserved_or_delay(&mut self, reserved_or_delay: [u8; 2]) {
        self.reserved_or_delay = reserved_or_delay;
    }
    // pub fn set_payload_len(&mut self, payload_len: usize) {
    //     self.payload_len = payload_len;
    // }
    pub fn add_payload_len(&mut self, add: usize) {
        self.payload_len += add;
    }
}

impl TryFrom<&[u8]> for DcgHead {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some(payload_len) = value
            .get(25)
            .and_then(|x| Some(to_u16(value[24], *x) as usize))
        {
            if PROFINET_ETHER_TYPE.0 != to_u16(value[12], value[13]) {
                bail!("todo");
            }
            let ty = PnDcpTy::try_from([value[14], value[15], value[16], value[17]])?;
            if payload_len + 26 > value.len() {
                bail!("payload({}) + 26 > {}", payload_len, value.len());
            }
            let destination =
                MacAddr::new(value[0], value[1], value[2], value[3], value[4], value[5]);
            let source = MacAddr::new(value[6], value[7], value[8], value[9], value[10], value[11]);
            let xid: [u8; 4] = [value[18], value[19], value[20], value[21]];
            let reserved_or_delay: [u8; 2] = [value[22], value[23]];
            return Ok(DcgHead {
                destination,
                source,
                ty,
                xid,
                reserved_or_delay,
                payload_len,
            });
        }
        bail!("长度不足，非PnDcg包");
    }
}

// pub const FRAME_ID_DCP_HELLO: FrameId = FrameId(0xfe, 0xfc);
// pub const FRAME_ID_DCP_GETORSET: FrameId = FrameId(0xfe, 0xfd);
// pub const FRAME_ID_DCP_IDENT_REQ: FrameId = FrameId(0xfe, 0xfe);
// pub const FRAME_ID_DCP_IDENT_RES: FrameId = FrameId(0xfe, 0xff);

pub enum FrameId {
    Hello,
    GetOrSet,
    IdentReq,
    IdentResp,
    UnSupport(u8, u8),
}
impl From<[u8; 2]> for FrameId {
    fn from(a: [u8; 2]) -> Self {
        match a {
            [0xfe, 0xfc] => Self::Hello,
            [0xfe, 0xfd] => Self::GetOrSet,
            [0xfe, 0xfe] => Self::IdentReq,
            [0xfe, 0xff] => Self::IdentResp,
            [a, b] => Self::UnSupport(a, b),
        }
    }
}

// pub const SERVICE_ID_GET: u8 = 0x03;
// pub const SERVICE_ID_SET: u8 = 0x04;
// pub const SERVICE_ID_IDENTIFY: u8 = 0x05;
// pub const SERVICE_ID_HELLO: u8 = 0x06;

// #[repr(u8)]
// pub enum ServiceId {
//     Get = 0x03,
//     Set = 0x04,
//     Identify = 0x05,
//     Hello = 0x06,
// }

// pub const SERVICE_TYPE_REQUEST: u8 = 0x00;
// pub const SERVICE_TYPE_RESPONSE_SUCCESS: u8 = 0x01;
// pub const SERVICE_TYPE_RESPONSE_UNSUPPORTED: u8 = 0x05;
