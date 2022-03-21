use crate::comm::BytesWrap;
use crate::dcp_block::BlockGetReq;
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::bail;

#[derive(Debug)]
pub struct PacketGetReq {
    pub head: DcgHead,
    pub blocks: BlockGetReq,
}

impl TryFrom<PnDcg> for PacketGetReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::GetReq {
            bail!("todo");
        }
        let blocks = BlockGetReq::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketGetReq {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketGetReq::try_from(dcg)
    }
}
