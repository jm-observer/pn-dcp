use crate::block::{BlockOptionAndSub, BlockTrait};
use crate::comm::BytesWrap;
use crate::options::OptionAndSub;
use crate::packet::{DcpHead, PnDcp, PnDcpTy};
use anyhow::bail;
use pn_dcg_macro::derefmut;
use pnet::util::MacAddr;
use std::ops::{Deref, DerefMut};
#[derive(Debug, Eq, PartialEq)]
#[derefmut(head)]
pub struct PacketGetReq {
    head: DcpHead,
    blocks: BlockGetReq,
}

impl PacketGetReq {
    pub fn new(source: MacAddr, dest: MacAddr) -> Self {
        let head = DcpHead::new(dest, source, PnDcpTy::GetReq);
        Self {
            head,
            blocks: BlockGetReq::default(),
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

impl TryFrom<PnDcp> for PacketGetReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcp) -> Result<Self, Self::Error> {
        let PnDcp { head, blocks } = dcg;
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
        let dcg = PnDcp::try_from(value)?;
        PacketGetReq::try_from(dcg)
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
#[derefmut(0)]
pub struct BlockGetReq(Vec<BlockOptionAndSub>);

impl BlockTrait for BlockGetReq {
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
        while let Ok(tmp) = value.slice(index..) {
            let one = OptionAndSub::try_from(tmp)?;
            blocks.push(one.into());
            index += 2;
        }
        Ok(blocks.into())
    }
}
