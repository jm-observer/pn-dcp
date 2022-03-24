pub mod comm;

use anyhow::{bail, Result};
use comm::*;
use pn_dcg_packet::pn_dcp::{PnDcg, PnDcgBuilder, PnDcpTy};

#[test]
fn test() -> Result<()> {
    assert!(test_dcp(get_ident_req(), PnDcpTy::IdentReq).is_ok());
    assert!(test_dcp(get_ident_resp(), PnDcpTy::IdentRespSuc).is_ok());
    assert!(test_dcp(get_get_req(), PnDcpTy::GetReq).is_ok());
    assert!(test_dcp(get_get_resp(), PnDcpTy::GetRespSuc).is_ok());
    assert!(test_dcp(get_set_req(), PnDcpTy::SetReq).is_ok());
    assert!(test_dcp(get_set_resp(), PnDcpTy::SetRespSuc).is_ok());
    Ok(())
}

fn test_dcp(data: Vec<u8>, ty: PnDcpTy) -> Result<()> {
    if let Ok(packet) = PnDcg::try_from(data.as_slice()) {
        if packet.head.ty != ty {
            bail!("");
        }
        if get_blocks(data.as_slice())? != packet.blocks.as_ref() {
            bail!("todo")
        }
    } else {
        bail!("todo");
    }
    Ok(())
}

#[test]
fn test_builder() -> Result<()> {
    assert!(test_dcp_builder(get_ident_req(), PnDcpTy::IdentReq).is_ok());
    assert!(test_dcp_builder(get_ident_resp(), PnDcpTy::IdentRespSuc).is_ok());
    assert!(test_dcp_builder(get_get_req(), PnDcpTy::GetReq).is_ok());
    assert!(test_dcp_builder(get_get_resp(), PnDcpTy::GetRespSuc).is_ok());
    assert!(test_dcp_builder(get_set_req(), PnDcpTy::SetReq).is_ok());
    assert!(test_dcp_builder(get_set_resp(), PnDcpTy::SetRespSuc).is_ok());
    Ok(())
}

fn test_dcp_builder(data: Vec<u8>, ty: PnDcpTy) -> Result<()> {
    let slice = data.as_slice();
    let pn_data = PnDcgBuilder::new(ty)
        .set_xid(get_xid(slice)?)
        .set_response_delay(get_response_delay(slice)?)
        .set_payload(get_blocks(slice)?.to_vec())
        .set_des_array(get_destination_array(slice)?)
        .set_src_array(get_src_array(slice)?)
        .build()?;
    if data != pn_data {
        println!("{:?}", data);
        println!("{:?}", pn_data);
        bail!("todo");
    }
    Ok(())
}
