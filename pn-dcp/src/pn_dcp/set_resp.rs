use crate::comm::BytesWrap;
use crate::dcp_block::{BlockResp, BlockSet};
use crate::options::{BlockError, OptionAndSub, OptionAndSubValue, Response};
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::bail;
use pn_dcg_macro::derefmut;
use pnet::datalink::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
#[derefmut(head)]
pub struct PacketSetResp {
    head: DcgHead,
    blocks: Response,
}

impl PacketSetResp {
    pub fn new(source: MacAddr, dest: MacAddr, option: OptionAndSub, error: BlockError) -> Self {
        let head = DcgHead::new(dest, source, PnDcpTy::SetReq);
        let blocks = Response(option, error);
        Self { head, blocks }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcg> for PacketSetResp {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::SetRespSuc {
            bail!("todo");
        }
        if OptionAndSub::try_from(blocks.slice(0..=1)?)? == OptionAndSub::Response {
            let blocks = Response::try_from(blocks.slice(4..=6)?)?;
            Ok(Self { head, blocks })
        } else {
            bail!("not a respose")
        }
    }
}

impl TryFrom<&[u8]> for PacketSetResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketSetResp::try_from(dcg)
    }
}
