use crate::block::{BlockCommon, BlockIp, BlockPadding, BlockResp, BlockTrait};
use crate::comm::BytesWrap;
use crate::options::IpBlockInfo;
use crate::options::{BlockError, BlockInfo, InnerIpAddr, OptionAndSub, OptionAndSubValue};
use crate::packet::{DcpHead, PnDcp, PnDcpTy};
use anyhow::{bail, Result};
use pn_dcp_macro::derefmut;
use pnet::util::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
#[derefmut(head)]
pub struct PacketGetResp {
    head: DcpHead,
    blocks: GetRespBlocks,
}
impl Deref for PacketGetResp {
    type Target = DcpHead;

    fn deref(&self) -> &Self::Target {
        &self.head
    }
}

impl PacketGetResp {
    pub fn new(source: MacAddr, dest: MacAddr) -> Self {
        let head = DcpHead::new(dest, source, PnDcpTy::GetRespSuc);
        Self {
            head,
            blocks: GetRespBlocks::default(),
        }
    }
    fn append_block(&mut self, option: impl Into<GetRespBlock>) {
        let block = option.into();
        let block_len = block.len();
        self.blocks.0.push(block);
        self.head.add_payload_len(block_len);
        if block_len % 2 == 1 {
            self.blocks.0.push(GetRespBlock::Padding(BlockPadding));
            self.head.add_payload_len(1);
        }
    }

    pub fn append_block_ip(&mut self, ip: InnerIpAddr, info: IpBlockInfo) {
        self.append_block(BlockIp { ip, info })
    }
    pub fn append_block_common(&mut self, option: OptionAndSubValue, info: BlockInfo) {
        self.append_block(BlockCommon { option, info })
    }
    pub fn append_block_resp(&mut self, option: OptionAndSub, error: BlockError) {
        self.append_block(BlockResp(option, error))
    }
    pub fn block_ip(&self) -> Result<BlockIp> {
        for block in self.blocks.iter() {
            if let GetRespBlock::BlockIp(ip) = block {
                return Ok(ip.clone());
            }
        }
        bail!("not contain ip info!");
    }
    pub fn block_commons(&self) -> Vec<BlockCommon> {
        let mut blocks = Vec::new();
        for block in self.blocks.iter() {
            if let GetRespBlock::Block(common) = block {
                blocks.push(common.clone());
            }
        }
        blocks
    }
    pub fn block_resps(&self) -> Vec<BlockResp> {
        let mut blocks = Vec::new();
        for block in self.blocks.iter() {
            if let GetRespBlock::BlockResp(common) = block {
                blocks.push(common.clone());
            }
        }
        blocks
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcp> for PacketGetResp {
    type Error = anyhow::Error;
    fn try_from(dcg: PnDcp) -> Result<Self, Self::Error> {
        let PnDcp { head, blocks } = dcg;
        if head.ty != PnDcpTy::GetRespSuc {
            bail!("the packet is pn-dcp, but not get resp success!");
        }
        let blocks = GetRespBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketGetResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcp::try_from(value)?;
        PacketGetResp::try_from(dcg)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GetRespBlock {
    Block(BlockCommon),
    BlockIp(BlockIp),
    BlockResp(BlockResp),
    Padding(BlockPadding),
}
#[derive(Debug, Eq, PartialEq, Default, Clone)]
#[derefmut(0)]
pub struct GetRespBlocks(pub(crate) Vec<GetRespBlock>);

impl BlockTrait for GetRespBlock {
    fn len(&self) -> usize {
        match self {
            Self::Block(a) => a.len(),
            Self::Padding(a) => a.len(),
            Self::BlockIp(a) => a.len(),
            Self::BlockResp(a) => a.len(),
        }
    }

    fn payload(&self) -> u16 {
        match self {
            Self::Block(a) => a.payload(),
            Self::Padding(a) => a.payload(),
            Self::BlockIp(a) => a.payload(),
            Self::BlockResp(a) => a.payload(),
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

    fn payload(&self) -> u16 {
        unreachable!()
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
            if tmp.len() == 0 {
                break;
            }
            let option = OptionAndSub::try_from(tmp.clone())?;
            let len = match option {
                OptionAndSub::IpAddr => {
                    let block = BlockIp::try_from_bytes(tmp)?;
                    // println!("{:?}", block);
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
