use crate::comm::BytesWrap;
use crate::dcp_block::{BlockOptionAndSub, BlockPadding, BlockTrait};
use crate::options::OptionAndSub;
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::bail;
use pn_dcg_macro::ImplDerefMutHead;
use pnet::util::MacAddr;
use std::ops::{Deref, DerefMut};
#[derive(Debug, Eq, PartialEq, ImplDerefMutHead)]
pub struct PacketGetReq {
    head: DcgHead,
    blocks: BlockGetReq,
}

impl PacketGetReq {
    pub fn new(source: MacAddr, dest: MacAddr) -> Self {
        let mut head = DcgHead::new(dest, source, PnDcpTy::GetReq);
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

#[derive(Debug, Eq, PartialEq, Default)]
pub struct BlockGetReq(Vec<BlockOptionAndSub>);

impl Deref for BlockGetReq {
    type Target = Vec<BlockOptionAndSub>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockGetReq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
