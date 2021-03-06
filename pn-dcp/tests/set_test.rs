mod comm;

use anyhow::Result;
use comm::*;
use pn_dcp::block::{BlockResp, BlockSet};
use pn_dcp::options::{BlockError, BlockQualifier, InnerIpAddr, OptionAndSub};
use pn_dcp::packet::set_req::PacketSetReq;
use pn_dcp::packet::set_resp::PacketSetResp;
use std::net::Ipv4Addr;

#[test]
fn test_req() -> Result<()> {
    let data = get_set_req();
    let set = PacketSetReq::try_from(data.as_slice())?;
    let src = get_src_array(data.as_slice()).unwrap();
    let dest = get_destination_array(data.as_slice()).unwrap();

    let ip = InnerIpAddr::new_by_ipv4(
        Ipv4Addr::from(0xc0a8c7f5),
        Ipv4Addr::from(0xffffff00),
        Ipv4Addr::from(0xc0a8c7fe),
    );

    let mut get_req = PacketSetReq::new(
        init_mac_by_array(src),
        init_mac_by_array(dest),
        ip.clone().to_option(),
        BlockQualifier::SavePermanent,
    );
    get_req.set_xid(get_xid(data.as_slice()).unwrap());

    assert_eq!(
        get_req.blocks(),
        BlockSet::new(ip.to_option(), BlockQualifier::SavePermanent)
    );

    assert_eq!(data, get_req.to_vec());
    assert_eq!(set, get_req);
    Ok(())
}

#[test]
fn test_resp() -> Result<()> {
    let data = get_set_resp();
    let set = PacketSetResp::try_from(data.as_slice())?;

    let src = get_src_array(data.as_slice()).unwrap();
    let dest = get_destination_array(data.as_slice()).unwrap();

    let mut get_req = PacketSetResp::new(
        init_mac_by_array(src),
        init_mac_by_array(dest),
        OptionAndSub::IpAddr,
        BlockError::Ok,
    );
    get_req.set_xid(get_xid(data.as_slice()).unwrap());

    assert_eq!(
        get_req.blocks(),
        vec![BlockResp(OptionAndSub::IpAddr, BlockError::Ok)]
    );

    assert_eq!(data, get_req.to_vec());
    assert_eq!(set, get_req);
    Ok(())
}

#[test]
fn test_get_ext() -> Result<()> {
    let req = PacketSetReq::try_from(get_set_req().as_slice())?;
    let req_ext = PacketSetReq::try_from(get_set_req_ext().as_slice())?;
    assert_eq!(req, req_ext);

    let resp = PacketSetResp::try_from(get_set_resp().as_slice())?;
    let resp_ext = PacketSetResp::try_from(get_set_resp_ext().as_slice())?;
    assert_eq!(resp, resp_ext);

    Ok(())
}
