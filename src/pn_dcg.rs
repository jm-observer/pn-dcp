use crate::profinet::FrameId;
use pnet::packet::ethernet::EtherType;
// use pnet::packet::PacketSize;
use pnet::util::MacAddr;
use pnet_macros::packet;
use pnet_macros_support::packet::PrimitiveValues;
use pnet_macros_support::types::{u16be, u32be};

type ServiceId = u8;
type Reserved = DoubleU8s;

#[packet]
pub struct PnDcg {
    #[construct_with(u8, u8, u8, u8, u8, u8)]
    pub destination: MacAddr,
    #[construct_with(u8, u8, u8, u8, u8, u8)]
    pub source: MacAddr,
    #[construct_with(u16)]
    pub ethertype: EtherType,
    #[construct_with(u8, u8)]
    pub frame_id: FrameId,
    pub service_id: u8,
    pub service_type: u8,
    pub xid: u32be,
    #[construct_with(u8, u8)]
    pub reserved: Reserved,
    pub dcp_len: u16be,
    #[payload]
    #[length = "dcp_len"]
    pub payload: Vec<u8>,
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
