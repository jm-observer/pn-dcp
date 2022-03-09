#![allow(dead_code)]

use pnet::packet::ethernet::EtherType;

pub const PROFINET_ETHER_TYPE: EtherType = EtherType(0x8892);

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

// xid 4 Bytes
// ResponseDelay 2 Bytes
// DCPDataLength 2 Bytes
// Block

// #[test]
// fn test_u32() {
//     let a = [1u8, 0, 0, 0];
//     let u32v = U32Packet::new(&a).unwrap();
//     println!("{:0x}", u32v);
// }
