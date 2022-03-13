use crate::block::{Blocks, OptionAndSub, OptionSuboptions};
use crate::consts::*;
use anyhow::{anyhow, bail};
use pnet::packet::ethernet::EthernetPacket;
use pnet_macros::packet;
// use pnet_macros_support::packet::Packet;
use crate::comm::{group_copy_to_vec, slice_copy_to_vec, u16_to_u8s};
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
impl<'a> DcpPacket<'a> {
    pub fn is_set_req(&self) -> bool {
        match self {
            Self::SetReq(_) => true,
            _ => false,
        }
    }
    pub fn is_set_resp(&self) -> bool {
        match self {
            Self::SetResp(_) => true,
            _ => false,
        }
    }
    pub fn is_get_resp(&self) -> bool {
        match self {
            Self::GetResp(_) => true,
            _ => false,
        }
    }
    pub fn is_get_req(&self) -> bool {
        match self {
            Self::GetReq(_) => true,
            _ => false,
        }
    }
    pub fn is_ident_resp(&self) -> bool {
        match self {
            Self::IdentResp(_) => true,
            _ => false,
        }
    }
    pub fn is_ident_req(&self) -> bool {
        match self {
            Self::IdentReq(_) => true,
            _ => false,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for DcpPacket<'a> {
    type Error = anyhow::Error;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let ethernet = EthernetPacket::<'a>::new(data).ok_or(anyhow!("not a ethernet packet"))?;
        if ethernet.get_ethertype() != PROFINET_ETHER_TYPE {
            bail!("");
        }
        let index = pnet_macros_support::packet::PacketSize::packet_size(&ethernet);
        let payload = &data[index..];
        let profinet = ProfinetPacket::new(payload).ok_or(anyhow!("not a profinet packet"))?;
        let frame_id: FrameId = profinet.get_frame_id();
        let service_id: u8 = profinet.get_service_id();
        let service_ty: u8 = profinet.get_service_type();

        let index = pnet_macros_support::packet::PacketSize::packet_size(&profinet);
        let payload = &payload[index..];
        if frame_id == FRAME_ID_DCP_HELLO {
            bail!("not support hello packet yet")
        } else if frame_id == FRAME_ID_DCP_GETORSET {
            match (service_id, service_ty) {
                (SERVICE_ID_GET, SERVICE_TYPE_REQUEST) => {
                    let blocks = OptionSuboptions::new(payload);
                    return Ok(Self::GetReq(DcpGetReq {
                        ethernet,
                        profinet,
                        blocks,
                    }));
                }
                (SERVICE_ID_GET, SERVICE_TYPE_RESPONSE_SUCCESS) => {
                    let blocks = Blocks::new(payload);
                    return Ok(Self::GetResp(DcpCommPacket {
                        ethernet,
                        profinet,
                        blocks,
                    }));
                }
                (SERVICE_ID_SET, SERVICE_TYPE_REQUEST) => {
                    let blocks = Blocks::new(payload);
                    return Ok(Self::SetReq(DcpCommPacket {
                        ethernet,
                        profinet,
                        blocks,
                    }));
                }
                (SERVICE_ID_SET, SERVICE_TYPE_RESPONSE_SUCCESS) => {
                    let blocks = Blocks::new(payload);
                    return Ok(Self::SetResp(DcpCommPacket {
                        ethernet,
                        profinet,
                        blocks,
                    }));
                }
                _ => {
                    bail!("unidentified packet")
                }
            }
        } else if frame_id == FRAME_ID_DCP_IDENT_REQ {
            if (service_id, service_ty) == (SERVICE_ID_IDENTIFY, SERVICE_TYPE_REQUEST) {
                let blocks = Blocks::new(payload);
                return Ok(Self::IdentReq(DcpCommPacket {
                    ethernet,
                    profinet,
                    blocks,
                }));
            } else {
                bail!("unidentified packet")
            }
        } else if frame_id == FRAME_ID_DCP_IDENT_RES {
            if (service_id, service_ty) == (SERVICE_ID_IDENTIFY, SERVICE_TYPE_RESPONSE_SUCCESS) {
                let blocks = Blocks::new(payload);
                return Ok(Self::IdentResp(DcpCommPacket {
                    ethernet,
                    profinet,
                    blocks,
                }));
            } else {
                bail!("unidentified packet")
            }
        } else {
            bail!("unidentified packet")
        }
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

pub struct BlocksBuilder(Vec<u8>);

impl BlocksBuilder {
    pub fn append_block(mut self, option_and_sub: OptionAndSub, payload: &[u8]) -> Self {
        // let mut datas = Vec::with_capacity(payload.len() + 4);
        // group_copy_to_vec(&mut datas, &option_and_sub.to_u8s());
        // slice_copy_to_vec(&mut datas, &u16_to_u8s((payload.len() + 4) as u16));
        // slice_copy_to_vec(&mut datas, payload);
        // self.0.push(datas);
        todo!();
    }
    pub fn build<'a>(self) -> Blocks<'a> {
        // let mut blocks = Blocks::default();
        todo!();
        // for i in self.0 {
        //     blocks.append_block(i);
        // }
    }
}
