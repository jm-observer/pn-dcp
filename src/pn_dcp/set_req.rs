use crate::comm::BytesWrap;
use crate::dcp_block::{BlockGetReq, BlockSet};
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::bail;

#[derive(Debug)]
pub struct PacketSetReq {
    pub head: DcgHead,
    pub blocks: BlockSet,
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
