use pnet_macros::packet;
use pnet_macros_support::packet::PrimitiveValues;
use pnet_macros_support::types::{u16be, u32be};

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
