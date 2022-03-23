use crate::comm::{to_u16, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use crate::dcp_block::{
    BlockCommon, BlockCommonWithoutInfo, BlockIp, BlockPadding, BlockResp, BlockTrait,
};
use crate::options::{OptionAndSub, OptionAndSubValue};
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::{bail, Result};
use bytes::Bytes;
use pnet::util::MacAddr;

#[derive(Debug)]
pub struct PacketGetResp {
    pub head: DcgHead,
    pub blocks: GetRespBlocks,
}

impl TryFrom<PnDcg> for PacketGetResp {
    type Error = anyhow::Error;
    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::GetRespSuc {
            bail!("todo");
        }
        let blocks = GetRespBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketGetResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketGetResp::try_from(dcg)
    }
}

#[derive(Debug)]
pub enum GetRespBlock {
    Block(BlockCommon),
    BlockIp(BlockIp),
    BlockResp(BlockResp),
    Padding(BlockPadding),
}
#[derive(Debug)]
pub struct GetRespBlocks(pub Vec<GetRespBlock>);
impl BlockTrait for GetRespBlock {
    fn len(&self) -> usize {
        match self {
            Self::Block(a) => a.len(),
            Self::Padding(a) => a.len(),
            Self::BlockIp(a) => a.len(),
            Self::BlockResp(a) => a.len(),
        }
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::Block(a) => a.append_data(data),
            Self::Padding(a) => a.append_data(data),
            Self::BlockIp(a) => a.append_data(data),
            Self::BlockResp(a) => a.append_data(data),
        }
    }
}

impl BlockTrait for GetRespBlocks {
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

impl TryFrom<BytesWrap> for GetRespBlocks {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<GetRespBlock>::new();
        while let Ok(tmp) = value.slice(index..) {
            println!("{:?}", tmp.as_ref());
            let option = OptionAndSub::try_from(tmp.clone())?;
            let len = match option {
                OptionAndSub::IpAddr => {
                    let block = BlockIp::try_from_bytes(tmp)?;
                    println!("{:?}", block);
                    let len = block.len();
                    blocks.push(block.into());
                    len
                }
                OptionAndSub::Response => {
                    let block = BlockResp::try_from(tmp)?;
                    let len = block.len();
                    blocks.push(block.into());
                    len
                }
                option => {
                    let block = BlockCommon::try_from_bytes(option, tmp)?;
                    println!("{:?}", block);
                    let len = block.len();
                    blocks.push(block.into());
                    len
                }
            };
            if len % 2 == 1 {
                blocks.push(BlockPadding.into());
                index += 1;
            }
            index += len;
        }
        Ok(blocks.into())
    }
}
