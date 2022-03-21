mod comm;
use anyhow::Result;
use comm::*;
use pn_dcg_packet::pn_dcp::set_req::PacketSetReq;
use pn_dcg_packet::pn_dcp::set_resp::PacketSetResp;

#[test]
fn test_req() -> Result<()> {
    let data = get_set_req();
    let set = PacketSetReq::try_from(data.as_slice())?;
    println!("{:?}", set);
    Ok(())
}

#[test]
fn test_resp() -> Result<()> {
    let data = get_set_resp();
    let set = PacketSetResp::try_from(data.as_slice())?;
    println!("{:?}", set);
    Ok(())
}
