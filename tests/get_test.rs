mod comm;
use anyhow::Result;
use comm::*;
use pn_dcg_packet::pn_dcp::get_req::PacketGetReq;
use pn_dcg_packet::pn_dcp::get_resp::PacketGetResp;

#[test]
fn test_get() -> Result<()> {
    let data = get_get_req();
    let get = PacketGetReq::try_from(data.as_slice())?;
    println!("{:?}", get);
    Ok(())
}

#[test]
fn test_get_resp() -> Result<()> {
    let data = get_get_resp();
    let get = PacketGetResp::try_from(data.as_slice())?;
    println!("{:?}", get);
    Ok(())
}
