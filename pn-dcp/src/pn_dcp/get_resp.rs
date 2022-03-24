use crate::comm::{to_u16, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use crate::dcp_block::{
    BlockCommon, BlockCommonWithoutInfo, BlockIp, BlockPadding, BlockResp, BlockTrait,
};
use crate::options::{OptionAndSub, OptionAndSubValue};
use crate::pn_dcp::get_req::PacketGetReq;
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::{bail, Result};
use bytes::Bytes;
use pn_dcg_macro::ImplDerefMutHead;
use pnet::util::MacAddr;
use std::ops::{Deref, DerefMut};
#[derive(Debug, Eq, PartialEq, ImplDerefMutHead)]
pub struct PacketGetResp {
    head: DcgHead,
    blocks: GetRespBlocks,
}

impl PacketGetResp {
    pub fn new(get_req: &PacketGetReq) -> Self {
        let mut head = DcgHead::new(
            get_req.source.clone(),
            get_req.destination.clone(),
            PnDcpTy::GetRespSuc,
        );
        Self {
            head,
            blocks: GetRespBlocks::default(),
        }
    }
    pub fn append_block(&mut self, option: OptionAndSub) {
        self.blocks.push(option.into());
        self.head.add_payload_len(2);
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
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

#[derive(Debug, Eq, PartialEq)]
pub enum GetRespBlock {
    Block(BlockCommon),
    BlockIp(BlockIp),
    BlockResp(BlockResp),
    Padding(BlockPadding),
}
#[derive(Debug, Eq, PartialEq, Default)]
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