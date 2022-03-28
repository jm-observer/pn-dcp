use crate::block::{BlockCommon, BlockIp, BlockPadding, BlockTrait};
use crate::comm::BytesWrap;
use crate::options::IpBlockInfo;
use crate::options::{BlockInfo, InnerIpAddr, OptionAndSub, OptionAndSubValue};
use crate::packet::ident_req::PacketIdentReq;
use crate::packet::{DcpHead, PnDcp, PnDcpTy};
use anyhow::{bail, Result};
use pn_dcg_macro::derefmut;
use pnet::util::MacAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdentRespBlock {
    Block(BlockCommon),
    BlockIp(BlockIp),
    Padding(BlockPadding),
}

impl IdentRespBlock {
    pub fn add_to_packet(self, packet: &mut PacketIdentResp) {
        packet.append_block(self);
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
#[derefmut(0)]
pub struct IdentRespBlocks(Vec<IdentRespBlock>);
impl IdentRespBlocks {
    pub fn from(data: Vec<IdentRespBlock>) -> Self {
        Self(data)
    }
}

impl BlockTrait for IdentRespBlocks {
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

impl BlockTrait for IdentRespBlock {
    fn len(&self) -> usize {
        match self {
            Self::Block(a) => a.len(),
            Self::BlockIp(a) => a.len(),
            Self::Padding(a) => a.len(),
        }
    }

    fn payload(&self) -> u16 {
        match self {
            Self::Block(a) => a.payload(),
            Self::BlockIp(a) => a.payload(),
            Self::Padding(a) => a.payload(),
        }
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::Padding(a) => a.append_data(data),
            Self::BlockIp(a) => a.append_data(data),
            Self::Block(a) => a.append_data(data),
        }
    }
}

impl TryFrom<BytesWrap> for IdentRespBlocks {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<IdentRespBlock>::new();
        while let Ok(tmp) = value.slice(index..) {
            let option = OptionAndSub::try_from(tmp.clone())?;
            let len = match option {
                OptionAndSub::IpAddr => {
                    let block = BlockIp::try_from_bytes(tmp)?;
                    let len = block.len();
                    blocks.push(block.into());
                    len
                }
                OptionAndSub::Response => {
                    bail!("todo OptionAndSub::Response")
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
#[derive(Debug, Eq, PartialEq, Clone)]
#[derefmut(head)]
pub struct PacketIdentResp {
    head: DcpHead,
    blocks: IdentRespBlocks,
}

impl PacketIdentResp {
    pub fn new(source: MacAddr, dest: MacAddr) -> Self {
        let head = DcpHead::new(dest, source, PnDcpTy::IdentRespSuc);
        Self {
            head,
            blocks: IdentRespBlocks::default(),
        }
    }
    pub fn from_req(source: MacAddr, ident_req: PacketIdentReq) -> Self {
        let mut head = DcpHead::new(ident_req.source.clone(), source, PnDcpTy::IdentRespSuc);
        head.set_xid(ident_req.xid);
        Self {
            head,
            blocks: IdentRespBlocks::default(),
        }
    }
    fn append_block(&mut self, block: impl Into<IdentRespBlock>) {
        let block = block.into();
        let block_len = block.len();
        self.blocks.0.push(block);
        self.head.add_payload_len(block_len);
        if block_len % 2 == 1 {
            self.blocks.0.push(IdentRespBlock::Padding(BlockPadding));
            self.head.add_payload_len(1);
        }
    }
    pub fn append_block_ip(&mut self, ip: InnerIpAddr, info: IpBlockInfo) {
        self.append_block(BlockIp { ip, info })
    }
    pub fn append_block_common(&mut self, option: OptionAndSubValue, info: BlockInfo) {
        self.append_block(BlockCommon { option, info })
    }
    pub fn append_block_common_default(&mut self, option: OptionAndSubValue) {
        self.append_block(BlockCommon {
            option,
            info: BlockInfo::Reserved,
        })
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.head.payload_len + 26);
        self.head.append_data(&mut data);
        self.blocks.append_data(&mut data);
        data
    }
}

impl TryFrom<PnDcp> for PacketIdentResp {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcp) -> Result<Self, Self::Error> {
        let PnDcp { head, blocks } = dcg;
        if head.ty != PnDcpTy::IdentRespSuc {
            bail!("todo");
        }
        let blocks = IdentRespBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketIdentResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcp::try_from(value)?;
        PacketIdentResp::try_from(dcg)
    }
}
