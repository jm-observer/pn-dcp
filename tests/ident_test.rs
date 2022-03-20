mod comm;
use anyhow::Result;
use comm::*;
use pn_dcg_packet::pn_dcp::ident_req::PacketIdentReq;
use pn_dcg_packet::pn_dcp::ident_resp::PacketIdentResp;

#[test]
fn ident_req_test() -> Result<()> {
    let ident_req_data = get_ident_req();
    let req = PacketIdentReq::try_from(ident_req_data.as_slice())?;
    println!("{:0x?}", req.get_manufacturer_pecific_block()?.as_ref());
    println!("{:0x?}", get_blocks(ident_req_data.as_slice())?);
    Ok(())
}
#[test]
fn ident_resp_test() -> Result<()> {
    let ident_req_data = get_ident_resp();
    let req = PacketIdentResp::try_from(ident_req_data.as_slice())?;
    println!("{:0x?}", req.blocks);
    println!("{:0x?}", get_blocks(ident_req_data.as_slice())?);
    Ok(())
}
