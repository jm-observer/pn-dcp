use crate::dcp_block::{BlockCommon, BlockCommonWithoutInfo, BlockIp, BlockPadding, BlockResp};
use crate::options::OptionAndSubValue;
use crate::pn_dcp::get_resp::{GetRespBlock, GetRespBlocks, PacketGetResp};
use crate::pn_dcp::ident_req::{IdentReqBlock, IdentReqBlocks};
use crate::pn_dcp::ident_resp::{IdentRespBlock, IdentRespBlocks};
use crate::pn_dcp::{PnDcg, PnDcpTy};
use anyhow::bail;

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
        GetRespBlocks::from_vec(val)
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
