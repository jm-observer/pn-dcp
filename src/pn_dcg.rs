use crate::profinet::FrameId;
use pnet::packet::ethernet::EtherType;
use pnet::util::MacAddr;

pub struct PnDcg {
    #[construct_with(u8, u8, u8, u8, u8, u8)]
    pub destination: MacAddr,
    #[construct_with(u8, u8, u8, u8, u8, u8)]
    pub source: MacAddr,
    #[construct_with(u16)]
    pub ethertype: EtherType,
    pub frame_id: FrameId,
    pub service_id: u8,
    pub service_type: u8,
    pub xid: u32,
    pub reserved: u16,
    pub dcp_len: u16,
    pub payload: Vec<u8>,
}
