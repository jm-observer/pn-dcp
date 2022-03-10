use pnet::packet::PacketSize;
use pnet_macros::packet;
use pnet_macros_support::types::u16be;
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

pub enum OptionAndSub {
    Mar_Addr,
    Ip_Addr,
    Full_Ip_Suite,
    Manufacturer_Specific,
    Name_Of_Station,
    Device_Id,
    Device,
    Device_Options,
    Alias_Name,
    Start_Transaction,
    End_Transaction,
    Signal,
    Response,
    Reset_Factory,
    Devicec_Initiative,
    All,
    DHCP(u8),
    LLDP(u8),
    Other((u8, u8)),
}

impl OptionAndSub {
    pub fn get(a: (u8, u8)) -> Self {
        match a {
            (1, 1) => Self::Mar_Addr,
            (1, 2) => Self::Ip_Addr,
            (1, 3) => Self::Full_Ip_Suite,
            (2, 1) => Self::Manufacturer_Specific,
            (2, 2) => Self::Name_Of_Station,
            (2, 3) => Self::Device_Id,
            (2, 4) => Self::Device,
            (2, 5) => Self::Device_Options,
            (2, 6) => Self::Alias_Name,
            (5, 1) => Self::Start_Transaction,
            (5, 2) => Self::End_Transaction,
            (5, 3) => Self::Signal,
            (5, 4) => Self::Response,
            (5, 6) => Self::Reset_Factory,
            (6, 1) => Self::Devicec_Initiative,
            (255, 255) => Self::All,
            (3, a) => Self::DHCP(a),
            (4, a) => Self::LLDP(a),
            _ => Self::Other(a),
        }
    }
    pub fn to_u8s(&self) -> (u8, u8) {
        match *self {
            Self::Mar_Addr => (1, 1),
            Self::Ip_Addr => (1, 2),
            Self::Full_Ip_Suite => (1, 3),
            Self::Manufacturer_Specific => (2, 1),
            Self::Name_Of_Station => (2, 2),
            Self::Device_Id => (2, 3),
            Self::Device => (2, 4),
            Self::Device_Options => (2, 5),
            Self::Alias_Name => (2, 6),
            Self::Start_Transaction => (5, 1),
            Self::End_Transaction => (5, 2),
            Self::Signal => (5, 3),
            Self::Response => (5, 4),
            Self::Reset_Factory => (5, 6),
            Self::Devicec_Initiative => (6, 1),
            Self::All => (255, 255),
            Self::DHCP(a) => (3, a),
            Self::LLDP(a) => (4, a),
            Self::Other(a) => a,
        }
    }
}
