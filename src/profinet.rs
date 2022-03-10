use crate::block::{Blocks, OptionSuboptions};
use crate::consts::*;
use anyhow::bail;
use pnet::packet::ethernet::EthernetPacket;
use pnet_macros::packet;
use pnet_macros_support::packet::PrimitiveValues;
use pnet_macros_support::types::{u16be, u32be};

pub enum DcpPacket<'a> {
    IdentReq(DcpCommPacket<'a>),
    IdentResp(DcpCommPacket<'a>),
    GetReq(DcpGetReq<'a>),
    GetResp(DcpCommPacket<'a>),
    SetReq(DcpCommPacket<'a>),
    SetResp(DcpCommPacket<'a>),
}

impl<'a> TryFrom<EthernetPacket<'a>> for DcpPacket<'a> {
    type Error = anyhow::Error;

    fn try_from(value: EthernetPacket<'a>) -> Result<Self, Self::Error> {
        if packet.get_ethertype() != PROFINET_ETHER_TYPE {}
        let payload = value.payload();
        let profinet = ProfinetPacket::new(payload)?;

        let frame_id: FrameId = profinet.get_frame_id();
        if frame_id == FRAME_ID_DCP_HELLO {
            bail!("not support hello packet yet")
        } else if frame_id == FRAME_ID_DCP_GETORSET {
        } else if frame_id == FRAME_ID_DCP_IDENT_REQ {
        } else if frame_id == FRAME_ID_DCP_IDENT_RES {
        } else {
            bail!("unidentified packet")
        }
        todo!()
    }
}

pub struct DcpCommPacket<'a> {
    ethernet: EthernetPacket<'a>,
    profinet: ProfinetPacket<'a>,
    blocks: Blocks<'a>,
}

pub struct DcpGetReq<'a> {
    ethernet: EthernetPacket<'a>,
    profinet: ProfinetPacket<'a>,
    blocks: OptionSuboptions,
}

#[packet]
pub struct Profinet {
    #[construct_with(u8, u8)]
    pub frame_id: FrameId,
    pub service_id: u8,
    pub service_type: u8,
    pub xid: u32be,
    pub response_delay: u16be,
    pub dcp_data_length: u16be,
    #[payload]
    #[length = "dcp_data_length"]
    pub blocks: Vec<u8>,
}
#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd, Debug)]
pub struct FrameId(pub u8, pub u8);

impl FrameId {
    pub fn new(a: u8, b: u8) -> Self {
        Self(a, b)
    }

    pub fn to_u8s(&self) -> [u8; 2] {
        [self.0, self.1]
    }
}
impl PrimitiveValues for FrameId {
    type T = (u8, u8);

    fn to_primitive_values(&self) -> Self::T {
        (self.0, self.1)
    }
}
