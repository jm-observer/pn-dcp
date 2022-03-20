extern crate core;

mod comm;

use comm::*;
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

    let data = get_ident_resp();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);

    // let data = get_get_req();
    // let blocks_data = get_blocks(data.as_slice()).unwrap();
    // let blocks = Blocks::new(blocks_data);
    // println!("{:?}", blocks);

    let data = get_get_resp();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);

    let data = get_set_req();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);

    let data = get_set_resp();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);
}
#[test]
fn test_ident() {
    let data = get_ident_req();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);

    let data = get_ident_resp();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = Blocks::new(blocks_data);
    println!("{:?}", blocks);
}

#[test]
fn test_get_req() {
    let data = get_get_req();
    let blocks_data = get_blocks(data.as_slice()).unwrap();
    let blocks = OptionSuboptions::new(blocks_data);
    assert!(eq_option_and_sub(blocks.get(0), OptionAndSub::IpAddr));
    assert!(eq_option_and_sub(blocks.get(1), OptionAndSub::DHCP(61)));
}
fn eq_option_and_sub(val: Option<&OptionAndSub>, left: OptionAndSub) -> bool {
    if let Some(right) = val {
        right == &left
    } else {
        false
    }
}
