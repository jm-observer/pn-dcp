use crate::block::{
    BlockCommon, BlockCommonWithoutInfo, BlockIp, BlockPadding, BlockResp, BlockSet,
};
use crate::options::OptionAndSubValue;
use crate::packet::get_resp::{GetRespBlock, GetRespBlocks};
use crate::packet::ident_req::{IdentReqBlock, IdentReqBlocks};
use crate::packet::ident_resp::{IdentRespBlock, IdentRespBlocks};
use crate::packet::set_req::{SetReqBlock, SetReqBlocks};
use crate::packet::set_resp::{SetRespBlock, SetRespBlocks};

impl From<BlockCommon> for GetRespBlock {
    fn from(a: BlockCommon) -> Self {
        Self::Block(a)
    }
}
impl From<BlockIp> for GetRespBlock {
    fn from(a: BlockIp) -> Self {
        Self::BlockIp(a)
    }
}
impl From<BlockPadding> for GetRespBlock {
    fn from(a: BlockPadding) -> Self {
        Self::Padding(a)
    }
}
impl From<BlockResp> for GetRespBlock {
    fn from(a: BlockResp) -> Self {
        Self::BlockResp(a)
    }
}
impl From<Vec<GetRespBlock>> for GetRespBlocks {
    fn from(val: Vec<GetRespBlock>) -> Self {
        GetRespBlocks(val)
    }
}

impl From<BlockCommonWithoutInfo> for IdentReqBlock {
    fn from(a: BlockCommonWithoutInfo) -> Self {
        Self::Block(a)
    }
}
impl From<OptionAndSubValue> for IdentReqBlock {
    fn from(a: OptionAndSubValue) -> Self {
        Self::Block(BlockCommonWithoutInfo(a))
    }
}
impl From<BlockPadding> for IdentReqBlock {
    fn from(a: BlockPadding) -> Self {
        Self::Padding(a)
    }
}
impl From<Vec<IdentReqBlock>> for IdentReqBlocks {
    fn from(val: Vec<IdentReqBlock>) -> Self {
        IdentReqBlocks::from_vec(val)
    }
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
impl From<Vec<IdentRespBlock>> for IdentRespBlocks {
    fn from(val: Vec<IdentRespBlock>) -> Self {
        Self::from(val)
    }
}

impl From<BlockPadding> for SetRespBlock {
    fn from(a: BlockPadding) -> Self {
        Self::Padding(a)
    }
}
impl From<BlockResp> for SetRespBlock {
    fn from(a: BlockResp) -> Self {
        Self::Response(a)
    }
}
impl From<Vec<SetRespBlock>> for SetRespBlocks {
    fn from(val: Vec<SetRespBlock>) -> Self {
        Self(val)
    }
}

impl From<BlockPadding> for SetReqBlock {
    fn from(a: BlockPadding) -> Self {
        Self::Padding(a)
    }
}
impl From<BlockSet> for SetReqBlock {
    fn from(a: BlockSet) -> Self {
        Self::Set(a)
    }
}
impl From<Vec<SetReqBlock>> for SetReqBlocks {
    fn from(val: Vec<SetReqBlock>) -> Self {
        Self(val)
    }
}
