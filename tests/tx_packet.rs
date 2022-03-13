use anyhow::{bail, Result};
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{interfaces, Config, NetworkInterface};

pub mod comm;
use comm::*;

#[test]
#[ignore]
pub fn simulate_host() -> Result<()> {
    let index: u32 = 10;
    let interface = get_interface(index)?;
    if let Some(_src) = interface.mac {
        let cf = Config::default();
        let (mut tx, _rx) = match datalink::channel(&interface, cf) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => bail!("Unhandled channel type"),
            Err(e) => bail!(
                "An error occurred when creating the datalink channel: {}",
                e
            ),
        };
        let data = get_ident_req();
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
        let data = get_ident_resp();
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
        let data = get_set_req();
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
        let data = get_set_resp();
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
        let data = get_get_req();
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
        let data = get_get_resp();
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
    }
    Ok(())
}
fn get_interface(index: u32) -> Result<NetworkInterface> {
    for interface in interfaces() {
        println!("{}", interface);
    }
    for interface in interfaces() {
        if interface.index == index {
            return Ok(interface);
        }
    }
    bail!("不存在【index={}】的网络接口", index);
}
