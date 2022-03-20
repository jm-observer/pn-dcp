extern crate core;

mod comm;

use comm::*;
use pn_dcg_packet::block::BlockPacket;
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
    let data = [
        0x53, 0x37, 0x2d, 0x32, 0x30, 0x30, 0x20, 0x53, 0x4d, 0x41, 0x52, 0x54,
    ];
    let packet = get_block();
    let block = BlockPacket::new(&packet).unwrap();
    assert_eq!(block.get_option_and_sub().to_u8s(), (0x02u8, 0x01u8));
    assert_eq!(block.get_block_len(), 0x000cu16);
    assert_eq!(block.payload(), data.as_slice());
    assert_eq!(block.packet_size(), 16usize);

    let packet = get_block_with_padding();
    let block = BlockPacket::new(&packet).unwrap();
    assert_eq!(block.get_option_and_sub().to_u8s(), (0x02u8, 0x01u8));
    assert_eq!(block.get_block_len(), 0x000cu16);
    // assert_eq!(block.get_status(), 0x5337u16);
    assert_eq!(block.payload(), data.as_slice());
    assert_eq!(block.packet_size(), 16usize);
}

pub fn assert_eq_profinet_packet(packet: &EthernetPacket, origin: &[u8]) {
    assert_eq!(
        packet.get_destination().octets().as_ref(),
        get_destination_mac(origin).unwrap()
    );
    assert_eq!(
        packet.get_source().octets().as_ref(),
        get_src_mac(origin).unwrap()
    );
    let ether_type = u16_to_u8s(packet.get_ethertype().0);
    assert_eq!(ether_type.as_ref(), get_ethernet_type(origin).unwrap());
    let profinet: ProfinetPacket = ProfinetPacket::new(packet.payload()).unwrap();

    assert_eq!(
        profinet.get_frame_id().to_u8s().as_ref(),
        get_frame_id(origin).unwrap()
    );
    assert_eq!(&profinet.get_service_id(), get_service_id(origin).unwrap());
    assert_eq!(
        &profinet.get_service_type(),
        get_service_type(origin).unwrap()
    );
    assert_eq!(
        u32_to_u8s(profinet.get_xid()).as_slice(),
        get_xid(origin).unwrap()
    );

    assert_eq!(
        profinet.get_response_delay(),
        get_response_delay(origin).unwrap()
    );
    assert_eq!(
        u16_to_u8s(profinet.get_dcp_data_length()).as_slice(),
        get_dcp_data_length(origin).unwrap()
    );
}
