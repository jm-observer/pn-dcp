use crate::block::{BlockCommonWithoutInfo, BlockPadding, BlockTrait};
use crate::comm::BytesWrap;
use crate::options::OptionAndSubValue;
use crate::packet::{DcpHead, PnDcp, PnDcpTy};
use anyhow::{bail, Result};
use pn_dcp_macro::derefmut;
use pnet::util::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdentReqBlock {
    Block(BlockCommonWithoutInfo),
    Padding(BlockPadding),
}
#[derive(Debug, Eq, PartialEq, Clone)]
#[derefmut(0)]
pub struct IdentReqBlocks(Vec<IdentReqBlock>);

impl IdentReqBlocks {
    pub fn from_vec(val: Vec<IdentReqBlock>) -> Self {
        IdentReqBlocks(val)
    }
}

impl BlockTrait for IdentReqBlocks {
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

impl BlockTrait for IdentReqBlock {
    fn len(&self) -> usize {
        match self {
            Self::Block(a) => a.len(),
            Self::Padding(a) => a.len(),
        }
    }

    fn payload(&self) -> u16 {
        match self {
            Self::Block(a) => a.payload(),
            Self::Padding(a) => a.payload(),
        }
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::Padding(a) => a.append_data(data),
            Self::Block(a) => a.append_data(data),
        }
    }
}

impl Default for IdentReqBlocks {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl TryFrom<BytesWrap> for IdentReqBlocks {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<IdentReqBlock>::new();
        while let Ok(tmp) = value.slice(index..) {
            if tmp.len() == 0 {
                break;
            }
            let one = BlockCommonWithoutInfo::try_from(tmp)?;
            let len = one.len();
            if one.len() % 2 == 1 {
                blocks.push(one.into());
                blocks.push(BlockPadding.into());
                index += 1;
            } else {
                blocks.push(one.into());
            }
            index += len;
        }
        Ok(blocks.into())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[derefmut(head)]
pub struct PacketIdentReq {
    head: DcpHead,
    blocks: IdentReqBlocks,
}
impl Deref for PacketIdentReq {
    type Target = DcpHead;

    fn deref(&self) -> &Self::Target {
        &self.head
    }
}
impl PacketIdentReq {
    pub fn new(source: MacAddr) -> Self {
        let destination = MacAddr::new(0x01, 0x0e, 0xcf, 0x00, 0x00, 0x00);
        let head = DcpHead::new(destination, source, PnDcpTy::IdentReq);
        Self {
            head,
            blocks: IdentReqBlocks::default(),
        }
    }
    fn append_block(&mut self, block: impl Into<IdentReqBlock>) {
        let block = block.into();
        let block_len = block.len();
        self.blocks.0.push(block);
        self.head.add_payload_len(block_len);
        if block_len % 2 == 1 {
            self.blocks.0.push(IdentReqBlock::Padding(BlockPadding));
            self.head.add_payload_len(1);
        }
    }
    pub fn append_block_by_option(&mut self, option: OptionAndSubValue) {
        self.append_block(BlockCommonWithoutInfo(option));
    }

    pub fn options(&self) -> Vec<OptionAndSubValue> {
        let mut options = Vec::new();
        for option in self.blocks.iter() {
            if let IdentReqBlock::Block(BlockCommonWithoutInfo(option)) = option {
                options.push(option.clone());
            }
        }
        options
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcp> for PacketIdentReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcp) -> Result<Self, Self::Error> {
        let PnDcp { head, blocks } = dcg;
        if head.ty != PnDcpTy::IdentReq {
            bail!("the packet is pn-dcp, but not ident req!");
        }
        let blocks = IdentReqBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketIdentReq {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcp::try_from(value)?;
        PacketIdentReq::try_from(dcg)
    }
}
