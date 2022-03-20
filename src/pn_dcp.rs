pub mod ident_req;
pub mod ident_resp;

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
    pub fn to_u8s(&self) -> [u8; 4] {
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

        slice_copy_to_vec(&mut data, &ty.to_u8s());
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
    pub ty: PnDcpTy,
    pub header: BytesWrap,
    pub blocks: BytesWrap,
}
impl PnDcg {
    // pub fn payload(&self) -> &[u8] {
    //     self.data.split_off() & self.data[26..]
    // }
}

impl TryFrom<&[u8]> for PnDcg {
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
            let mut header: BytesWrap = value.to_vec().into();
            let payload = header.split_off(26)?;
            return Ok(Self {
                header,
                ty,
                blocks: payload,
            });
        }
        bail!("长度不足，非PnDcg包");
    }
}
