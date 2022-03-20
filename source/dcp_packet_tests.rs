pub mod comm;

use anyhow::anyhow;
use comm::*;

#[test]
fn test_ident() {
    let data = get_ident_req();
    let res = DcpPacket::try_from(data.as_slice());
    assert!(res.is_ok());
    assert!(res
        .and_then(|packet| {
            if packet.is_ident_req() {
                Ok(())
            } else {
                Err(anyhow!(""))
            }
        })
        .is_ok());
    let data = get_ident_resp();
    let res = DcpPacket::try_from(data.as_slice());
    assert!(res.is_ok());
    assert!(res
        .and_then(|packet| {
            if packet.is_ident_resp() {
                Ok(())
            } else {
                Err(anyhow!(""))
            }
        })
        .is_ok());
}

#[test]
fn test_get() {
    let data = get_get_req();
    let res = DcpPacket::try_from(data.as_slice());
    assert!(res.is_ok());
    assert!(res
        .and_then(|packet| {
            if packet.is_get_req() {
                Ok(())
            } else {
                Err(anyhow!(""))
            }
        })
        .is_ok());
    let data = get_get_resp();
    let res = DcpPacket::try_from(data.as_slice());
    assert!(res.is_ok());
    assert!(res
        .and_then(|packet| {
            if packet.is_get_resp() {
                Ok(())
            } else {
                Err(anyhow!(""))
            }
        })
        .is_ok());
}

#[test]
fn test_set() {
    let data = get_set_req();
    let res = DcpPacket::try_from(data.as_slice());
    assert!(res.is_ok());
    assert!(res
        .and_then(|packet| {
            if packet.is_set_req() {
                Ok(())
            } else {
                Err(anyhow!(""))
            }
        })
        .is_ok());
    let data = get_set_resp();
    let res = DcpPacket::try_from(data.as_slice());
    assert!(res.is_ok());
    assert!(res
        .and_then(|packet| {
            if packet.is_set_resp() {
                Ok(())
            } else {
                Err(anyhow!(""))
            }
        })
        .is_ok());
}
