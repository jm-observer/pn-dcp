use crate::comm::{to_u16, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use crate::dcp_block::{BlockCommonWithoutInfo, BlockPadding, BlockTrait};
use crate::options::OptionAndSubValue;
use crate::pn_dcp::ident_req::IdentReqBlock::Padding;
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::{bail, Result};
use bytes::Bytes;
use pnet::util::MacAddr;

#[derive(Debug)]
pub enum IdentReqBlock {
    Block(BlockCommonWithoutInfo),
    Padding(BlockPadding),
}
#[derive(Debug)]
pub struct IdentReqBlocks(pub Vec<IdentReqBlock>);

impl BlockTrait for IdentReqBlocks {
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

impl BlockTrait for IdentReqBlock {
    fn len(&self) -> usize {
        match self {
            Self::Block(a) => a.len(),
            Self::Padding(a) => a.len(),
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
        println!("{:?}", value);
        while let Ok(tmp) = value.slice(index..) {
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

pub struct PacketIdentReq {
    pub head: DcgHead,
    pub blocks: IdentReqBlocks,
}

impl PacketIdentReq {
    pub fn new(source: MacAddr) -> Self {
        let destination = MacAddr::new(0x01, 0x0e, 0xcf, 0x00, 0x00, 0x00);
        let head = DcgHead::new(destination, source, PnDcpTy::IdentReq);
        Self {
            head,
            blocks: IdentReqBlocks::default(),
        }
    }
    pub fn set_xid(&mut self, xid: [u8; 4]) {
        self.head.set_xid(xid);
    }
    pub fn set_reserved_or_delay(&mut self, reserved_or_delay: [u8; 2]) {
        self.head.set_reserved_or_delay(reserved_or_delay);
    }
    pub fn append_block(&mut self, block: IdentReqBlock) {
        let block_len = block.len();
        self.blocks.0.push(block);
        self.head.add_payload_len(block_len);
        if block_len % 1 == 1 {
            self.blocks.0.push(IdentReqBlock::Padding(BlockPadding));
            self.head.add_payload_len(1);
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }

    // pub fn set_payload_len(&mut self, payload_len: usize) {
    //     self.payload_len = payload_len;
    // }
    pub fn get_manufacturer_pecific_block(&self) -> Result<BytesWrap> {
        for block in self.blocks.0.as_slice() {
            if let IdentReqBlock::Block(BlockCommonWithoutInfo(
                OptionAndSubValue::ManufacturerSpecific(data),
            )) = block
            {
                return Ok(data.clone());
            }
        }
        bail!("todo 未包含manufacturer_pecific信息")
    }
}

impl TryFrom<PnDcg> for PacketIdentReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::IdentReq {
            bail!("todo");
        }
        let blocks = IdentReqBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketIdentReq {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketIdentReq::try_from(dcg)
    }
}
