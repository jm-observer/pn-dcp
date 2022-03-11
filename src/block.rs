use pnet::packet::PacketSize;
use pnet_macros::packet;
use pnet_macros_support::types::u16be;
use anyhow::Result;
#[packet]
pub struct Block {
    pub option: u8,
    pub sub_option: u8,
    pub block_len: u16be,
    pub status: u16be,
    #[payload]
    #[length = "block_len - 2"]
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum BlockComm<'a> {
    Padding(u8),
    Block(BlockPacket<'a>),
}
#[derive(Debug)]
pub struct Blocks<'a>(Vec<BlockComm<'a>>);

impl<'a> Blocks<'a> {
    pub fn new(mut data: &'a [u8]) -> Self {
        println!("{:?}", data);
        let mut blocks: Vec<BlockComm<'a>> = Vec::new();
        while let Some(block) = BlockPacket::new(data) {
            if block.get_block_len() == 0 {
                break;
            }
            let mut len = block.packet_size();
            blocks.push(BlockComm::Block(block));
            if len % 2 == 1 {
                blocks.push(BlockComm::Padding(data[len]));
                len += 1;
            }
            if data.len() == len {
                break;
            } else {
                data = &data[len..]
            }
        }
        Self(blocks)
    }
}

// pub struct OptionSuboption(pub u8, pub u8);
pub struct OptionSuboptions(Vec<OptionAndSub>);

impl OptionSuboptions {
    pub fn new(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

pub enum OptionAndSub {
    MarAddr,
    IpAddr,
    FullIpSuite,
    ManufacturerSpecific,
    NameOfStation,
    DeviceId,
    Device,
    DeviceOptions,
    AliasName,
    StartTransaction,
    EndTransaction,
    Signal,
    Response,
    ResetFactory,
    DevicecInitiative,
    All,
    DHCP(u8),
    LLDP(u8),
    Other((u8, u8)),
}

impl OptionAndSub {
    pub fn get(a: (u8, u8)) -> Self {
        match a {
            (1, 1) => Self::MarAddr,
            (1, 2) => Self::IpAddr,
            (1, 3) => Self::FullIpSuite,
            (2, 1) => Self::ManufacturerSpecific,
            (2, 2) => Self::NameOfStation,
            (2, 3) => Self::DeviceId,
            (2, 4) => Self::Device,
            (2, 5) => Self::DeviceOptions,
            (2, 6) => Self::AliasName,
            (5, 1) => Self::StartTransaction,
            (5, 2) => Self::EndTransaction,
            (5, 3) => Self::Signal,
            (5, 4) => Self::Response,
            (5, 6) => Self::ResetFactory,
            (6, 1) => Self::DevicecInitiative,
            (255, 255) => Self::All,
            (3, a) => Self::DHCP(a),
            (4, a) => Self::LLDP(a),
            _ => Self::Other(a),
        }
    }
    pub fn to_u8s(&self) -> (u8, u8) {
        match *self {
            Self::MarAddr => (1, 1),
            Self::IpAddr => (1, 2),
            Self::FullIpSuite => (1, 3),
            Self::ManufacturerSpecific => (2, 1),
            Self::NameOfStation => (2, 2),
            Self::DeviceId => (2, 3),
            Self::Device => (2, 4),
            Self::DeviceOptions => (2, 5),
            Self::AliasName => (2, 6),
            Self::StartTransaction => (5, 1),
            Self::EndTransaction => (5, 2),
            Self::Signal => (5, 3),
            Self::Response => (5, 4),
            Self::ResetFactory => (5, 6),
            Self::DevicecInitiative => (6, 1),
            Self::All => (255, 255),
            Self::DHCP(a) => (3, a),
            Self::LLDP(a) => (4, a),
            Self::Other(a) => a,
        }
    }
}
