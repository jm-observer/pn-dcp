use crate::comm::{to_u16, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use crate::dcp_block::{BlockCommon, BlockCommonWithoutInfo, BlockIp, BlockPadding};
use crate::options::{OptionAndSub, OptionAndSubValue};
use crate::pn_dcp::{PnDcg, PnDcpTy};
use anyhow::{bail, Result};
use bytes::Bytes;
use pnet::util::MacAddr;

#[derive(Debug)]
pub enum IdentRespBlock {
    Block(BlockCommon),
    BlockIp(BlockIp),
    Padding(BlockPadding),
}
impl From<BlockCommon> for IdentRespBlock {
    fn from(a: BlockCommon) -> Self {
        Self::Block(a)
    }
}
impl From<BlockIp> for IdentRespBlock {
    fn from(a: BlockIp) -> Self {
        Self::BlockIp(a)
    }
}
impl From<BlockPadding> for IdentRespBlock {
    fn from(a: BlockPadding) -> Self {
        Self::Padding(a)
    }
}
#[derive(Debug)]
pub struct IdentRespBlocks(Vec<IdentRespBlock>);

impl From<Vec<IdentRespBlock>> for IdentRespBlocks {
    fn from(val: Vec<IdentRespBlock>) -> Self {
        Self(val)
    }
}

impl TryFrom<BytesWrap> for IdentRespBlocks {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<IdentRespBlock>::new();
        println!("{:?}", value);
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
                    bail!("todo OptionAndSub::Response")
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

pub struct PacketIdentResp {
    pub header: BytesWrap,
    pub blocks: IdentRespBlocks,
}

impl PacketIdentResp {
    pub fn get_manufacturer_pecific_block(&self) -> Result<BytesWrap> {
        // for block in self.blocks.0.as_slice() {
        //     if let IdentReqBlock::Block(BlockCommonWithoutInfo(
        //         OptionAndSubValue::ManufacturerSpecific(data),
        //     )) = block
        //     {
        //         return Ok(data.clone());
        //     }
        // }
        // bail!("todo 未包含manufacturer_pecific信息")
        todo!()
    }
}

impl TryFrom<PnDcg> for PacketIdentResp {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { ty, header, blocks } = dcg;
        if ty != PnDcpTy::IdentRespSuc {
            bail!("todo");
        }
        let blocks = IdentRespBlocks::try_from(blocks)?;
        Ok(Self { blocks, header })
    }
}

impl TryFrom<&[u8]> for PacketIdentResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketIdentResp::try_from(dcg)
    }
}
