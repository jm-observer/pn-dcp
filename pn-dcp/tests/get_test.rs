mod comm;

use anyhow::Result;
use comm::*;
use pn_dcp::block::{BlockIp, BlockResp};
use pn_dcp::options::IpBlockInfo;
use pn_dcp::options::{BlockError, InnerIpAddr, OptionAndSub};
use pn_dcp::packet::get_req::PacketGetReq;
use pn_dcp::packet::get_resp::PacketGetResp;
use std::net::Ipv4Addr;

#[test]
fn test_get() -> Result<()> {
    let data = get_get_req();
    let get = PacketGetReq::try_from(data.as_slice())?;
    let src = get_src_array(data.as_slice()).unwrap();
    let dest = get_destination_array(data.as_slice()).unwrap();

    let mut get_req = PacketGetReq::new(init_mac_by_array(src), init_mac_by_array(dest));
    get_req.set_xid(get_xid(data.as_slice()).unwrap());

    get_req.append_block(OptionAndSub::IpAddr);
    get_req.append_block(OptionAndSub::DHCP(0x3d));

    assert_eq!(
        get_req.options(),
        vec![OptionAndSub::IpAddr, OptionAndSub::DHCP(0x3d)]
    );

    assert_eq!(get, get_req);
    assert_eq!(data, get_req.to_vec());
    Ok(())
}

#[test]
fn test_get_resp() -> Result<()> {
    let data = get_get_resp();
    let get = PacketGetResp::try_from(data.as_slice())?;
    let src = get_src_array(data.as_slice()).unwrap();
    let dest = get_destination_array(data.as_slice()).unwrap();

    let mut get_req = PacketGetResp::new(init_mac_by_array(src), init_mac_by_array(dest));
    get_req.set_xid(get_xid(data.as_slice()).unwrap());

    let ip = InnerIpAddr::new_by_ipv4(
        Ipv4Addr::from(0xc0a8c7f5),
        Ipv4Addr::from(0xffffff00),
        Ipv4Addr::from(0xc0a8c7fe),
    );
    get_req.append_block_ip(ip.clone(), IpBlockInfo::IpSet);
    get_req.append_block_resp(
        OptionAndSub::DHCP(0x3d),
        BlockError::SuboptionUnsuppOrNoDataSetAvail,
    );

    assert_eq!(get_req.block_ip()?, BlockIp::new(ip, IpBlockInfo::IpSet));
    assert_eq!(
        get_req.block_resps(),
        vec![BlockResp(
            OptionAndSub::DHCP(0x3d),
            BlockError::SuboptionUnsuppOrNoDataSetAvail
        ),]
    );

    assert_eq!(data, get_req.to_vec());
    assert_eq!(get, get_req);
    Ok(())
}

#[test]
fn test_ext() -> Result<()> {
    let req = PacketGetReq::try_from(get_get_req().as_slice())?;
    let req_ext = PacketGetReq::try_from(get_get_req_ext().as_slice())?;
    assert_eq!(req, req_ext);

    let resp = PacketGetResp::try_from(get_get_resp().as_slice())?;
    let resp_ext = PacketGetResp::try_from(get_get_resp_ext().as_slice())?;
    assert_eq!(resp, resp_ext);

    Ok(())
}
