#![allow(dead_code)]

use crate::profinet::FrameId;
use pnet::packet::ethernet::EtherType;

pub const PROFINET_ETHER_TYPE: EtherType = EtherType(0x8892);

/// FrameID 2 byte
pub const FRAME_ID_DCP_HELLO: FrameId = FrameId(0xfe, 0xfc);
pub const FRAME_ID_DCP_GETORSET: FrameId = FrameId(0xfe, 0xfd);
pub const FRAME_ID_DCP_IDENT_REQ: FrameId = FrameId(0xfe, 0xfe);
pub const FRAME_ID_DCP_IDENT_RES: FrameId = FrameId(0xfe, 0xff);
/// ServiceID 1 byte
const SERVICE_ID_GET: u8 = 0x03;
const SERVICE_ID_SET: u8 = 0x04;
const SERVICE_ID_IDENTIFY: u8 = 0x05;
const SERVICE_ID_HELLO: u8 = 0x06;
/// Service-Type
const SERVICE_TYPE_REQUEST: u8 = 0x00;
const SERVICE_TYPE_RESPONSE_SUCCESS: u8 = 0x01;
const SERVICE_TYPE_RESPONSE_UNSUPPORTED: u8 = 0x05;

// const O_MAC_ADDR: OptionSuboption = OptionSuboption(1, 1);
// const O_IP_ADDR: OptionSuboption = OptionSuboption(1, 2);
// const O_FULL_IP_SUITE: OptionSuboption = OptionSuboption(1, 3);
// const O_MANUFACTURER_SPECIFIC: OptionSuboption = OptionSuboption(2, 1);
// const O_NAME_OF_STATION: OptionSuboption = OptionSuboption(2, 2);
// const O_DEVICE_ID: OptionSuboption = OptionSuboption(2, 3);
// const O_DEVICE: OptionSuboption = OptionSuboption(2, 4);
// const O_DEVICE_OPTIONS: OptionSuboption = OptionSuboption(2, 5);
// const O_ALIAS_NAME: OptionSuboption = OptionSuboption(2, 6);
//
// const O_START_TRANSACTION: OptionSuboption = OptionSuboption(5, 1);
// const O_END_TRANSACTION: OptionSuboption = OptionSuboption(5, 2);
// const O_SIGNAL: OptionSuboption = OptionSuboption(5, 3);
// const O_RESPONSE: OptionSuboption = OptionSuboption(5, 4);
// const O_RESET_TO_FACTORY: OptionSuboption = OptionSuboption(5, 6);
//
// const O_DEVICEC_INITIATIVE: OptionSuboption = OptionSuboption(6, 1);
// const O_ALL: OptionSuboption = OptionSuboption(255, 255);
