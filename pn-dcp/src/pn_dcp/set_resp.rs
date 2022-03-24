use crate::comm::BytesWrap;
use crate::dcp_block::{BlockResp, BlockSet};
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::bail;

#[derive(Debug, Eq, PartialEq)]
pub struct PacketSetResp {
    head: DcgHead,
    blocks: BlockResp,
}

impl TryFrom<PnDcg> for PacketSetResp {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::SetRespSuc {
            bail!("todo");
        }
        let blocks = BlockResp::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketSetResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketSetResp::try_from(dcg)
    }
}
