extern crate core;

mod comm;

use comm::*;
use pn_dcg_packet::block::BlockPacket;
use pn_dcg_packet::consts::PROFINET_ETHER_TYPE;
use pn_dcg_packet::profinet::ProfinetPacket;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::PacketSize;

#[test]
fn test() {
    let data = get_ident_req();
    let packet = EthernetPacket::new(data.as_slice());
    assert!(packet.is_some());
    let packet: EthernetPacket = packet.unwrap();
    assert_eq_profinet_packet(&packet, data.as_slice());

    let data = get_ident_resp();
    let packet = EthernetPacket::new(data.as_slice());
    assert!(packet.is_some());
    let packet: EthernetPacket = packet.unwrap();
    assert_eq_profinet_packet(&packet, data.as_slice());

    let data = get_get_req();
    let packet = EthernetPacket::new(data.as_slice());
    assert!(packet.is_some());
    let packet: EthernetPacket = packet.unwrap();
    assert_eq_profinet_packet(&packet, data.as_slice());

    let data = get_get_resp();
    let packet = EthernetPacket::new(data.as_slice());
    assert!(packet.is_some());
    let packet: EthernetPacket = packet.unwrap();
    assert_eq_profinet_packet(&packet, data.as_slice());

    let data = get_set_req();
    let packet = EthernetPacket::new(data.as_slice());
    assert!(packet.is_some());
    let packet: EthernetPacket = packet.unwrap();
    assert_eq_profinet_packet(&packet, data.as_slice());

    let data = get_set_resp();
    let packet = EthernetPacket::new(data.as_slice());
    assert!(packet.is_some());
    let packet: EthernetPacket = packet.unwrap();
    assert_eq_profinet_packet(&packet, data.as_slice());
}

#[test]
fn test_block_packet() {
    let data = [0x2d, 0x32, 0x30, 0x30, 0x20, 0x53, 0x4d, 0x41, 0x52, 0x54];
    let packet = get_block();
    let block = BlockPacket::new(&packet).unwrap();
    assert_eq!(block.get_option(), 0x02u8);
    assert_eq!(block.get_sub_option(), 0x01u8);
    assert_eq!(block.get_len(), 0x000cu16);
    assert_eq!(block.get_status(), 0x5337u16);
    assert_eq!(block.payload(), data.as_slice());
    assert_eq!(block.packet_size(), 16usize);

    let packet = get_block_with_padding();
    let block = BlockPacket::new(&packet).unwrap();
    assert_eq!(block.get_option(), 0x02u8);
    assert_eq!(block.get_sub_option(), 0x01u8);
    assert_eq!(block.get_len(), 0x000cu16);
    assert_eq!(block.get_status(), 0x5337u16);
    assert_eq!(block.payload(), data.as_slice());
    assert_eq!(block.packet_size(), 16usize);
}
