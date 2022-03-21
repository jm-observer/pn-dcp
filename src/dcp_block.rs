use crate::comm::{to_u16, BytesWrap};
use crate::options::ip::IpBlockInfo;
use crate::options::{BlockError, BlockInfo, BlockQualifier, OptionAndSub, OptionAndSubValue};
use anyhow::{bail, Result};
use bytes::Bytes;

#[derive(Debug)]
pub struct BlockPadding;
#[derive(Debug)]
pub struct BlockGetReq(Vec<OptionAndSub>);

impl From<Vec<OptionAndSub>> for BlockGetReq {
    fn from(a: Vec<OptionAndSub>) -> Self {
        Self(a)
    }
}
impl TryFrom<BytesWrap> for BlockGetReq {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<OptionAndSub>::new();
        println!("{:?}", value);
        while let Ok(tmp) = value.slice(index..) {
            let one = OptionAndSub::try_from(tmp)?;
            blocks.push(one);
            index += 2;
        }
        Ok(blocks.into())
    }
}

#[derive(Debug)]
pub struct BlockIp {
    option: OptionAndSubValue,
    info: IpBlockInfo,
}
impl BlockIp {
    pub fn try_from_bytes(value: BytesWrap) -> Result<Self> {
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let info = IpBlockInfo::try_from(value.slice(4..=5)?)?;
        let val = value.slice(6..(len.0 + 4))?;
        let option = OptionAndSubValue::init_by_ty(OptionAndSub::IpAddr, val)?;
        Ok(Self { option, info })
    }
    pub fn len(&self) -> usize {
        self.option.payload_size() + 6
    }
}

#[derive(Debug)]
pub struct BlockSet {
    option: OptionAndSubValue,
    qualifier: BlockQualifier,
}
impl TryFrom<BytesWrap> for BlockSet {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let ty = OptionAndSub::try_from(value.clone())?;
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let qualifier = BlockQualifier::try_from(value.slice(4..=5)?)?;
        let val = value.slice(6..(len.0 + 4))?;
        let option = OptionAndSubValue::init_by_ty(ty, val)?;
        Ok(Self { option, qualifier })
    }
}
impl BlockSet {
    pub fn len(&self) -> usize {
        self.option.payload_size() + 6
    }
}

#[derive(Debug)]
pub struct BlockCommon {
    option: OptionAndSubValue,
    info: BlockInfo,
}

impl BlockCommon {
    pub fn try_from_bytes(ty: OptionAndSub, value: BytesWrap) -> Result<Self> {
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let info = BlockInfo::try_from(value.slice(4..=5)?)?;
        let val = value.slice(6..(len.0 + 4))?;
        let option = OptionAndSubValue::init_by_ty(ty, val)?;
        Ok(Self { option, info })
    }
    pub fn len(&self) -> usize {
        self.option.payload_size() + 6
    }
}

#[derive(Debug)]
pub struct BlockResp(pub OptionAndSub, pub BlockError);
impl BlockResp {
    pub fn len(&self) -> usize {
        7
    }
}
impl TryFrom<BytesWrap> for BlockResp {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let ty = OptionAndSub::try_from(value.slice(4..=5)?)?;
        let val = value.slice(6..=6)?.as_ref()[0];
        Ok(Self(ty, BlockError::try_from(val)?))
    }
}

#[derive(Debug)]
pub struct BlockCommonWithoutInfo(pub OptionAndSubValue);

impl TryFrom<BytesWrap> for BlockCommonWithoutInfo {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let ty = OptionAndSub::try_from(value.clone())?;
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let val = value.slice(4..(len.0 + 4))?;
        Ok(Self(OptionAndSubValue::init_by_ty(ty, val)?))
    }
}

impl BlockCommonWithoutInfo {
    pub fn len(&self) -> usize {
        self.0.payload_size() + 4
    }
}

pub struct Len(pub usize);
impl TryFrom<&[u8]> for Len {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some(val) = value.get(0..=1) {
            let len = to_u16(val[0], val[1]) as usize;
            Ok(Len(len))
        } else {
            bail!("")
        }
    }
}
