use pnet_macros::packet;
use pnet_macros_support::types::u16be;

#[packet]
pub struct Padding {}

#[packet]
pub struct Block {
    pub option: u8,
    pub sub_option: u8,
    pub len: u16be,
    pub status: u16be,
    #[payload]
    #[length = "len - 2"]
    pub data: Vec<u8>,
}

pub enum BlockComm {
    Padding(u8),
    Block(Block),
}

pub struct Blocks(Vec<BlockComm>);
