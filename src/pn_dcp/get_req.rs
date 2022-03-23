use crate::comm::BytesWrap;
use crate::dcp_block::{BlockOptionAndSub, BlockTrait};
use crate::options::OptionAndSub;
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

#[derive(Debug)]
pub struct BlockGetReq(Vec<BlockOptionAndSub>);

impl BlockTrait for BlockGetReq {
    fn len(&self) -> usize {
        let mut len = 0;
        for block in &self.0 {
            len += block.len();
        }
        len
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        for block in &self.0 {
            block.append_data(data)
        }
    }
}

impl From<Vec<BlockOptionAndSub>> for BlockGetReq {
    fn from(a: Vec<BlockOptionAndSub>) -> Self {
        Self(a)
    }
}

impl TryFrom<BytesWrap> for BlockGetReq {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<BlockOptionAndSub>::new();
        println!("{:?}", value);
        while let Ok(tmp) = value.slice(index..) {
            let one = OptionAndSub::try_from(tmp)?;
            blocks.push(one.into());
            index += 2;
        }
        Ok(blocks.into())
    }
}
