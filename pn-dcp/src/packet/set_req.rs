use crate::block::{BlockPadding, BlockSet, BlockTrait};
use crate::comm::BytesWrap;
use crate::options::{BlockQualifier, OptionAndSubValue};
use crate::packet::{DcpHead, PnDcp, PnDcpTy};
use anyhow::bail;
use pn_dcg_macro::derefmut;
use pnet::datalink::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
#[derefmut(head)]
pub struct PacketSetReq {
    head: DcpHead,
    blocks: BlockSet,
}

impl PacketSetReq {
    pub fn new(
        source: MacAddr,
        dest: MacAddr,
        option: OptionAndSubValue,
        qualifier: BlockQualifier,
    ) -> Self {
        let mut head = DcpHead::new(dest, source, PnDcpTy::SetReq);
        let blocks = BlockSet { option, qualifier };
        head.add_payload_len(blocks.len());
        Self { head, blocks }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcp> for PacketSetReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcp) -> Result<Self, Self::Error> {
        let PnDcp { head, blocks } = dcg;
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
        let dcg = PnDcp::try_from(value)?;
        PacketSetReq::try_from(dcg)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SetReqBlock {
    Set(BlockSet),
    Padding(BlockPadding),
}

impl BlockTrait for SetReqBlock {
    fn len(&self) -> usize {
        match self {
            Self::Set(a) => a.len(),
            Self::Padding(a) => a.len(),
        }
    }
    fn payload(&self) -> u16 {
        match self {
            Self::Set(a) => a.payload(),
            Self::Padding(a) => a.payload(),
        }
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::Padding(a) => a.append_data(data),
            Self::Set(a) => a.append_data(data),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
#[derefmut(0)]
pub struct SetReqBlocks(pub(crate) Vec<SetReqBlock>);

impl TryFrom<BytesWrap> for SetReqBlocks {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<SetReqBlock>::new();
        while let Ok(tmp) = value.slice(index..) {
            let block = BlockSet::try_from(tmp)?;
            let len = block.len();
            blocks.push(block.into());
            if len % 2 == 1 {
                blocks.push(BlockPadding.into());
                index += 1;
            }
            index += len;
        }
        Ok(blocks.into())
    }
}
