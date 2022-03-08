#![allow(dead_code)]
use pnet_macros::packet;
use pnet_macros_support::packet::PrimitiveValues;
use pnet_macros_support::types::{u16be, u32be};
/// FrameID 2 byte
const FRAME_ID_DCP_HELLO: [u8; 2] = [0xfe, 0xfc];
const FRAME_ID_DCP_GETORSET: [u8; 2] = [0xfe, 0xfd];
const FRAME_ID_DCP_IDENT_REQ: [u8; 2] = [0xfe, 0xfe];
const FRAME_ID_DCP_IDENT_RES: [u8; 2] = [0xfe, 0xff];
/// ServiceID 1 byte
const PNDCP_SERVICE_ID_GET: u8 = 0x03;
const PNDCP_SERVICE_ID_SET: u8 = 0x04;
const PNDCP_SERVICE_ID_IDENTIFY: u8 = 0x05;
const PNDCP_SERVICE_ID_HELLO: u8 = 0x06;
/// Service-Type
const PNDCP_SERVICE_TYPE_REQUEST: u8 = 0x00;
const PNDCP_SERVICE_TYPE_RESPONSE_SUCCESS: u8 = 0x01;
const PNDCP_SERVICE_TYPE_RESPONSE_UNSUPPORTED: u8 = 0x05;

/// xid 4 Bytes
/// ResponseDelay 2 Bytes
/// DCPDataLength 2 Bytes
/// Block

#[packet]
pub struct Ethernet {
    #[construct_with(u8, u8)]
    pub frame_id: FrameId,
    // #[construct_with(u8, u8, u8, u8, u8, u8)]
    pub service_id: u8,
    // // #[construct_with(u16)]
    pub service_type: u8,
    // #[construct_with(u8, u8, u8, u8)]
    pub xid: u32be,
    // #[construct_with(u16)]
    pub response_delay: u16be,
    // #[construct_with(u16)]
    pub dcp_data_length: u16be,
    #[payload]
    #[length = "dcp_data_length"]
    pub block: Vec<u8>,
}
#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd, Debug)]
pub struct FrameId(pub u8, pub u8);

impl FrameId {
    pub fn new(a: u8, b: u8) -> Self {
        Self(a, b)
    }
}
impl PrimitiveValues for FrameId {
    type T = (u8, u8);

    fn to_primitive_values(&self) -> Self::T {
        (self.0, self.1)
    }
}

#[test]
fn test_u32() {
    let a = [1u8, 0, 0, 0];
    let u32v = U32Packet::new(&a).unwrap();
    println!("{:0x}", u32v);
}
