mod comm;
use anyhow::Result;
use comm::*;
use pn_dcp::comm::BytesWrap;
use pn_dcp::options::{OptionAndSub, OptionAndSubValue};
use pn_dcp::pn_dcp::ident_req::PacketIdentReq;
use pn_dcp::pn_dcp::ident_resp::PacketIdentResp;
use pnet::util::MacAddr;

#[test]
fn ident_req_test() -> Result<()> {
    let ident_req_data = get_ident_req();
    let req = PacketIdentReq::try_from(ident_req_data.as_slice())?;
    let src = get_src_array(ident_req_data.as_slice()).unwrap();
    let mut man = PacketIdentReq::new(MacAddr::new(src[0], src[1], src[2], src[3], src[4], src[5]));
    man.set_xid(get_xid(ident_req_data.as_slice()).unwrap());
    man.set_reserved_or_delay(
        get_response_delay(ident_req_data.as_slice())
            .unwrap()
            .to_be_bytes(),
    );
    let data = "S7-200 SMART".as_bytes().to_vec();
    let manufacturer =
        OptionAndSubValue::init_by_ty(OptionAndSub::ManufacturerSpecific, BytesWrap::from(data))?;
    man.append_block(manufacturer);
    assert_eq!(req, man);
    assert_eq!(ident_req_data, man.to_vec());
    // println!("{:0x?}", req.get_manufacturer_pecific_block()?.as_ref());
    // println!("{:0x?}", get_blocks(ident_req_data.as_slice())?);
    Ok(())
}
#[test]
fn ident_resp_test() -> Result<()> {
    let ident_req_data = get_ident_resp();
    let req = PacketIdentResp::try_from(ident_req_data.as_slice())?;
    // println!("{:0x?}", req.blocks);
    println!("{:0x?}", get_blocks(ident_req_data.as_slice())?);
    Ok(())
}
