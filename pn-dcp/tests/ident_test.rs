mod comm;

use anyhow::Result;
use comm::*;
use pn_dcp::block::{BlockCommon, BlockIp};
use pn_dcp::options::IpBlockInfo;
use pn_dcp::options::OptionAndSub::{AliasName, DeviceOptions, DeviceRole, IpAddr, MarAddr};
use pn_dcp::options::{DeviceOptionsBuilder, InnerIpAddr, OptionAndSub, OptionAndSubValue};
use pn_dcp::packet::ident_req::PacketIdentReq;
use pn_dcp::packet::ident_resp::PacketIdentResp;
use pnet::util::MacAddr;
use std::net::Ipv4Addr;

#[test]
fn ident_req_test() -> Result<()> {
    let ident_req_data = get_ident_req();
    let req = PacketIdentReq::try_from(ident_req_data.as_slice())?;
    let src = get_src_array(ident_req_data.as_slice()).unwrap();
    let xid = get_xid(ident_req_data.as_slice()).unwrap();
    let reserved_or_delay = get_response_delay(ident_req_data.as_slice())
        .unwrap()
        .to_be_bytes();

    let mut man = PacketIdentReq::new(MacAddr::new(src[0], src[1], src[2], src[3], src[4], src[5]));
    man.set_xid(xid);
    man.set_reserved_or_delay(reserved_or_delay);
    let manufacturer = OptionAndSubValue::ManufacturerSpecific("S7-200 SMART".as_bytes().into());
    man.append_block_by_option(manufacturer.clone());

    assert_eq!(man.options(), vec![manufacturer]);
    assert_eq!(req, man);
    assert_eq!(ident_req_data, man.to_vec());
    // println!("{:0x?}", req.get_manufacturer_pecific_block()?.as_ref());
    // println!("{:0x?}", get_blocks(ident_req_data.as_slice())?);
    Ok(())
}
#[test]
fn ident_resp_test() -> Result<()> {
    let ident_resp_data = get_ident_resp();
    let req = PacketIdentResp::try_from(ident_resp_data.as_slice())?;
    let src = get_src_array(ident_resp_data.as_slice()).unwrap();
    let dest = get_destination_array(ident_resp_data.as_slice()).unwrap();
    let mut resp = PacketIdentResp::new(init_mac_by_array(src), init_mac_by_array(dest));
    resp.set_xid(get_xid(ident_resp_data.as_slice()).unwrap());
    let manufacturer = OptionAndSubValue::ManufacturerSpecific("S7-200 SMART".as_bytes().into());
    resp.append_block_common_default(manufacturer.clone());

    let device_options = DeviceOptionsBuilder::default()
        .append_option(OptionAndSub::ManufacturerSpecific)
        .append_option(OptionAndSub::NameOfStation)
        .append_option(OptionAndSub::DeviceId)
        .append_option(DeviceRole)
        .append_option(DeviceOptions)
        .append_option(AliasName)
        .append_option(MarAddr)
        .append_option(IpAddr)
        .append_option(OptionAndSub::DHCP(61))
        .build();
    device_options
        .clone()
        .append_to_ident_resp_default(&mut resp);
    let name = OptionAndSubValue::NameOfStation("bb-abci.111".as_bytes().into());
    name.clone().append_to_ident_resp_default(&mut resp);
    let device_id = OptionAndSubValue::DeviceId([0x00, 0x2a], [0x00, 0x00]);
    device_id.clone().append_to_ident_resp_default(&mut resp);
    let device_role = OptionAndSubValue::DeviceRole(0x02, 0x00);
    device_role.clone().append_to_ident_resp_default(&mut resp);
    let ip = InnerIpAddr::new_by_ipv4(
        Ipv4Addr::from(0xc0a8c7f5),
        Ipv4Addr::from(0xffffff00),
        Ipv4Addr::from(0xc0a8c7fe),
    );
    ip.clone()
        .append_to_ident_resp(&mut resp, IpBlockInfo::IpSet);

    assert_eq!(resp.block_ip()?, BlockIp::new(ip, IpBlockInfo::IpSet));
    assert_eq!(
        resp.block_commons(),
        vec![
            BlockCommon::new(manufacturer),
            BlockCommon::new(device_options),
            BlockCommon::new(name),
            BlockCommon::new(device_id),
            BlockCommon::new(device_role)
        ]
    );
    assert_eq!(ident_resp_data, resp.to_vec());
    assert_eq!(req, resp);
    Ok(())
}

#[test]
fn ident_req_ext_test() -> Result<()> {
    let ident_req_data = get_ident_req_ext();
    let req_ext = PacketIdentReq::try_from(ident_req_data.as_slice())?;
    let req = PacketIdentReq::try_from(get_ident_req().as_slice())?;
    assert_eq!(req, req_ext);
    Ok(())
}

#[test]
fn ident_resp_ext_test() -> Result<()> {
    let ident_resp_data = get_ident_resp();
    let resp = PacketIdentResp::try_from(ident_resp_data.as_slice())?;
    let resp_ext = PacketIdentResp::try_from(get_ident_resp_ext().as_slice())?;
    assert_eq!(resp, resp_ext);
    Ok(())
}
