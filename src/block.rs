use pnet::packet::PacketSize;
use pnet_macros::packet;
use pnet_macros_support::types::u16be;
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

#[derive(Debug)]
pub enum BlockComm<'a> {
    Padding(u8),
    Block(BlockPacket<'a>),
}
#[derive(Debug)]
pub struct Blocks<'a>(Vec<BlockComm<'a>>);

impl<'a> Blocks<'a> {
    pub fn new(mut data: &'a [u8]) -> Self {
        let mut blocks: Vec<BlockComm<'a>> = Vec::new();
        while let Some(block) = BlockPacket::new(data) {
            if block.get_len() == 0 {
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
