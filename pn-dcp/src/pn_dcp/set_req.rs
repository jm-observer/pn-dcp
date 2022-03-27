use crate::comm::BytesWrap;
use crate::dcp_block::{BlockSet, BlockTrait};
use crate::options::{BlockQualifier, OptionAndSubValue};
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::bail;
use pn_dcg_macro::derefmut;
use pnet::datalink::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
#[derefmut(head)]
pub struct PacketSetReq {
    head: DcgHead,
    blocks: BlockSet,
}

impl PacketSetReq {
    pub fn new(
        source: MacAddr,
        dest: MacAddr,
        option: OptionAndSubValue,
        qualifier: BlockQualifier,
    ) -> Self {
        let mut head = DcgHead::new(dest, source, PnDcpTy::SetReq);
        let blocks = BlockSet { option, qualifier };
        head.add_payload_len(blocks.len());
        Self { head, blocks }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcg> for PacketSetReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::SetReq {
            bail!("todo");
        }
        let blocks = BlockSet::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketSetReq {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketSetReq::try_from(dcg)
    }
}
