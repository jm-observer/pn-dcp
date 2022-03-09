extern crate core;

mod comm;

use comm::*;
use pn_dcg_packet::block::{BlockPacket, Blocks};
use pn_dcg_packet::consts::PROFINET_ETHER_TYPE;
use pn_dcg_packet::profinet::ProfinetPacket;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::PacketSize;

#[test]
fn test() {
    let data = get_ident_req();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);
}
