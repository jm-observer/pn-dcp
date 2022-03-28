use crate::block::{BlockPadding, BlockResp, BlockTrait};
use crate::comm::BytesWrap;
use crate::options::{BlockError, OptionAndSub};
use crate::packet::{DcpHead, PnDcp, PnDcpTy};
use anyhow::bail;
use pn_dcp_macro::derefmut;
use pnet::datalink::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
#[derefmut(head)]
pub struct PacketSetResp {
    head: DcpHead,
    blocks: SetRespBlocks,
}
impl Deref for PacketSetResp {
    type Target = DcpHead;

    fn deref(&self) -> &Self::Target {
        &self.head
    }
}
#[derive(Default, Debug, Eq, PartialEq, Clone)]
#[derefmut(0)]
pub struct SetRespBlocks(pub(crate) Vec<SetRespBlock>);

impl TryFrom<BytesWrap> for SetRespBlocks {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<SetRespBlock>::new();
        while let Ok(tmp) = value.slice(index..) {
            if tmp.len() == 0 {
                break;
            }
            let option = BlockResp::try_from(tmp.clone())?;
            let len = option.len();
            blocks.push(option.into());
            if len % 2 == 1 {
                blocks.push(BlockPadding.into());
                index += 1;
            }
            index += len;
        }
        Ok(blocks.into())
    }
}

impl BlockTrait for SetRespBlocks {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SetRespBlock {
    Response(BlockResp),
    Padding(BlockPadding),
}

impl BlockTrait for SetRespBlock {
    fn len(&self) -> usize {
        match self {
            Self::Response(a) => a.len(),
            Self::Padding(a) => a.len(),
        }
    }
    fn payload(&self) -> u16 {
        match self {
            Self::Response(a) => a.payload(),
            Self::Padding(a) => a.payload(),
        }
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::Padding(a) => a.append_data(data),
            Self::Response(a) => a.append_data(data),
        }
    }
}

impl PacketSetResp {
    pub fn new(source: MacAddr, dest: MacAddr, option: OptionAndSub, error: BlockError) -> Self {
        let head = DcpHead::new(dest, source, PnDcpTy::SetRespSuc);
        let blocks = BlockResp(option, error);
        let mut resp = Self {
            head,
            blocks: SetRespBlocks::default(),
        };
        resp.append_block(blocks);
        resp
    }
    fn append_block(&mut self, block: impl Into<SetRespBlock>) {
        let block = block.into();
        let block_len = block.len();
        self.blocks.0.push(block);
        self.head.add_payload_len(block_len);
        if block_len % 2 == 1 {
            self.blocks.0.push(SetRespBlock::Padding(BlockPadding));
            self.head.add_payload_len(1);
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcp> for PacketSetResp {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcp) -> Result<Self, Self::Error> {
        let PnDcp { head, blocks } = dcg;
        if head.ty != PnDcpTy::SetRespSuc {
            bail!("the packet is pn-dcp, but not set resp success!");
        }
        let blocks = SetRespBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketSetResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcp::try_from(value)?;
        PacketSetResp::try_from(dcg)
    }
}
