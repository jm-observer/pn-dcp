pub mod extend_trait;
pub mod get_req;
pub mod get_resp;
pub mod ident_req;
pub mod ident_resp;
pub mod set_req;
pub mod set_resp;

use crate::comm::BytesWrap;
use crate::comm::PROFINET_ETHER_TYPE;
use anyhow::{bail, Result};
use pnet::util::MacAddr;

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
            _ => bail!("not a pn-dcp type!"),
        }
    }
}
pub struct PnDcp {
    pub head: DcpHead,
    pub blocks: BytesWrap,
}

impl TryFrom<&[u8]> for PnDcp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let head = DcpHead::try_from(value)?;
        if let Some(blocks_data) = value.get(26..(26 + head.payload_len)) {
            let blocks: BytesWrap = blocks_data.to_vec().into();
            return Ok(Self { head, blocks });
        }
        bail!("长度不足，非PnDcg包");
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DcpHead {
    pub destination: MacAddr,
    pub source: MacAddr,
    pub ty: PnDcpTy,
    pub xid: [u8; 4],
    pub reserved_or_delay: [u8; 2],
    pub payload_len: usize,
}

impl DcpHead {
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

impl TryFrom<&[u8]> for DcpHead {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some(payload_len) = value
            .get(25)
            .and_then(|x| Some(u16::from_be_bytes([value[24], *x]) as usize))
        {
            if PROFINET_ETHER_TYPE.0 != u16::from_be_bytes([value[12], value[13]]) {
                bail!("the packet is not a pn-dcp!");

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
            return Ok(DcpHead {
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
