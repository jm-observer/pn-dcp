use crate::comm::BytesWrap;
use crate::options::IpBlockInfo;
use crate::options::{
    BlockError, BlockInfo, BlockQualifier, InnerIpAddr, OptionAndSub, OptionAndSubValue,
};
use anyhow::{bail, Result};

pub trait BlockTrait {
    fn len(&self) -> usize;
    fn payload(&self) -> u16;
    fn append_data(&self, data: &mut Vec<u8>);
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockPadding;

impl BlockTrait for BlockPadding {
    fn len(&self) -> usize {
        1
    }

    fn payload(&self) -> u16 {
        0
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        data.push(0u8);
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockOptionAndSub(pub(crate) OptionAndSub);

impl From<OptionAndSub> for BlockOptionAndSub {
    fn from(a: OptionAndSub) -> Self {
        Self(a)
    }
}
impl BlockTrait for BlockOptionAndSub {
    fn len(&self) -> usize {
        2
    }

    fn payload(&self) -> u16 {
        0
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        let (a, b) = self.0.to_u8s();
        data.push(a);
        data.push(b);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockIp {
    pub(crate) ip: InnerIpAddr,
    pub(crate) info: IpBlockInfo,
}
impl BlockIp {
    pub fn new(ip: InnerIpAddr, info: IpBlockInfo) -> Self {
        Self { ip, info }
    }
    pub fn ip(&self) -> &InnerIpAddr {
        &self.ip
    }
    pub fn info(&self) -> &IpBlockInfo {
        &self.info
    }
    pub fn try_from_bytes(value: BytesWrap) -> Result<Self> {
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let info = IpBlockInfo::try_from(value.slice(4..=5)?)?;
        let ip = InnerIpAddr::new(value.slice(6..(len.0 + 4))?)?;
        Ok(Self { ip, info })
    }
}
impl BlockTrait for BlockIp {
    fn len(&self) -> usize {
        self.ip.payload_size() + 6
    }

    fn payload(&self) -> u16 {
        14
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(OptionAndSub::IpAddr.to_u8_array().as_slice());
        data.extend_from_slice(&14u16.to_be_bytes());
        data.extend_from_slice(self.info.to_u8_array().as_slice());
        self.ip.append_value_to_data(data);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockSet {
    pub(crate) option: OptionAndSubValue,
    pub(crate) qualifier: BlockQualifier,
}

impl BlockSet {
    pub fn new(option: OptionAndSubValue, qualifier: BlockQualifier) -> Self {
        Self { option, qualifier }
    }
    pub fn option(&self) -> &OptionAndSubValue {
        &self.option
    }
    pub fn qualifier(&self) -> &BlockQualifier {
        &self.qualifier
    }
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

impl BlockTrait for BlockSet {
    fn len(&self) -> usize {
        self.option.payload_size() + 6
    }

    fn payload(&self) -> u16 {
        (self.option.payload_size() + 2) as u16
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        self.option.append_option_to_data(data);
        data.extend_from_slice(self.payload().to_be_bytes().as_slice());
        data.extend_from_slice(self.qualifier.to_u8_array().as_slice());
        self.option.append_value_to_data(data);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockCommon {
    pub(crate) option: OptionAndSubValue,
    pub(crate) info: BlockInfo,
}

impl BlockCommon {
    pub fn option(&self) -> &OptionAndSubValue {
        &self.option
    }
    pub fn info(&self) -> &BlockInfo {
        &self.info
    }
    pub fn new(option: OptionAndSubValue) -> Self {
        Self {
            option,
            info: BlockInfo::Reserved,
        }
    }
    pub fn set_block_info(&mut self, info: BlockInfo) {
        self.info = info;
    }
    pub fn try_from_bytes(ty: OptionAndSub, value: BytesWrap) -> Result<Self> {
        let val = value.slice(2..)?;
        let len = Len::try_from(val.as_ref())?;
        let info = BlockInfo::try_from(value.slice(4..=5)?)?;
        let val = value.slice(6..(len.0 + 4))?;
        let option = OptionAndSubValue::init_by_ty(ty, val)?;
        Ok(Self { option, info })
    }
}
impl BlockTrait for BlockCommon {
    fn len(&self) -> usize {
        self.option.payload_size() + 6
    }

    fn payload(&self) -> u16 {
        (self.option.payload_size() + 2) as u16
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        self.option.append_option_to_data(data);
        data.extend_from_slice(
            ((self.option.payload_size() + 2) as u16)
                .to_be_bytes()
                .as_slice(),
        );
        data.extend_from_slice(self.info.to_u8_array().as_slice());
        self.option.append_value_to_data(data);
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockResp(pub OptionAndSub, pub BlockError);
impl BlockTrait for BlockResp {
    fn len(&self) -> usize {
        7
    }

    fn payload(&self) -> u16 {
        3
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(OptionAndSub::Response.to_u8_array().as_slice());
        data.extend_from_slice(self.payload().to_be_bytes().as_slice());
        data.extend_from_slice(self.0.to_u8_array().as_slice());
        data.push(self.1 as u8);
    }
}

impl TryFrom<BytesWrap> for BlockResp {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let ty = OptionAndSub::try_from(value.slice(4..=5)?)?;
        let val = value.slice(6..=6)?.as_ref()[0];
        Ok(Self(ty, BlockError::try_from(val)?))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockCommonWithoutInfo(pub(crate) OptionAndSubValue);

impl From<OptionAndSubValue> for BlockCommonWithoutInfo {
    fn from(a: OptionAndSubValue) -> Self {
        Self(a)
    }
}

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

impl BlockTrait for BlockCommonWithoutInfo {
    fn len(&self) -> usize {
        self.0.payload_size() + 4
    }

    fn payload(&self) -> u16 {
        self.0.payload_size() as u16
    }

    fn append_data(&self, data: &mut Vec<u8>) {
        self.0.append_option_to_data(data);
        data.extend_from_slice((self.0.payload_size() as u16).to_be_bytes().as_slice());
        self.0.append_value_to_data(data);
    }
}

pub struct Len(pub usize);
impl TryFrom<&[u8]> for Len {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some(val) = value.get(0..=1) {
            let len = u16::from_be_bytes([val[0], val[1]]) as usize;
            Ok(Len(len))
        } else {
            bail!("")
        }
    }
}
