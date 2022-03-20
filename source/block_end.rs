#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct BlockPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableBlockPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> BlockPacket<'a> {
    /// Constructs a new BlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<BlockPacket<'p>> {
        if packet.len() >= BlockPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(BlockPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new BlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the BlockPacket will
    /// own its own data and the underlying buffer will be dropped when the BlockPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<BlockPacket<'static>> {
        if packet.len() >= BlockPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(BlockPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a BlockPacket to a BlockPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> BlockPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        BlockPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a BlockPacket to a BlockPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> BlockPacket<'a> {
        BlockPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize {
        6
    }
    /// The size (in bytes) of a Block instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Block) -> usize {
        6 + _packet.data.len()
    }
    /// Get the option field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_option(&self) -> u8 {
        let _self = self;
        let co = 0;
        (_self.packet[co] as u8)
    }
    /// Get the sub_option field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sub_option(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the len field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_len(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = (_self.packet[co + 1] as u16be) as u16be;
        b0 | b1
    }
    /// Get the status field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_status(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = (_self.packet[co + 1] as u16be) as u16be;
        b0 | b1
    }
}
impl<'a> MutableBlockPacket<'a> {
    /// Constructs a new MutableBlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableBlockPacket<'p>> {
        if packet.len() >= MutableBlockPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableBlockPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableBlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableBlockPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableBlockPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableBlockPacket<'static>> {
        if packet.len() >= MutableBlockPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableBlockPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableBlockPacket to a BlockPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> BlockPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        BlockPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableBlockPacket to a BlockPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> BlockPacket<'a> {
        BlockPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize {
        6
    }
    /// The size (in bytes) of a Block instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Block) -> usize {
        6 + _packet.data.len()
    }
    /// Populates a BlockPacket using a Block structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Block) {
        let _self = self;
        _self.set_option(packet.option);
        _self.set_sub_option(packet.sub_option);
        _self.set_len(packet.len);
        _self.set_status(packet.status);
        _self.set_data(&packet.data);
    }
    /// Get the option field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_option(&self) -> u8 {
        let _self = self;
        let co = 0;
        (_self.packet[co] as u8)
    }
    /// Get the sub_option field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sub_option(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the len field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_len(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = (_self.packet[co + 1] as u16be) as u16be;
        b0 | b1
    }
    /// Get the status field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_status(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = (_self.packet[co + 1] as u16be) as u16be;
        b0 | b1
    }
    /// Set the option field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_option(&mut self, val: u8) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the sub_option field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sub_option(&mut self, val: u8) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the len field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_len(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 0xff00) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the status field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_status(&mut self, val: u16be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 0xff00) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the data field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_data(&mut self, vals: &[u8]) {
        let mut _self = self;
        let current_offset = 6;
        let len = _self.get_len() as usize - 2;
        assert!(vals.len() <= len);
        _self.packet[current_offset..current_offset + vals.len()].copy_from_slice(vals);
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for BlockPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        6 + _self.get_len() as usize - 2
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableBlockPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        6 + _self.get_len() as usize - 2
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableBlockPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 6;
        let end = ::std::cmp::min(6 + _self.get_len() as usize - 2, _self.packet.len());
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..end]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableBlockPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 6;
        let end = ::std::cmp::min(6 + _self.get_len() as usize - 2, _self.packet.len());
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..end]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for BlockPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 6;
        let end = ::std::cmp::min(6 + _self.get_len() as usize - 2, _self.packet.len());
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `BlockPacket`s
pub struct BlockIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for BlockIterable<'a> {
    type Item = BlockPacket<'a>;
    fn next(&mut self) -> Option<BlockPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = BlockPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for BlockPacket<'p> {
    type T = Block;
    #[inline]
    fn from_packet(&self) -> Block {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Block {
            option: _self.get_option(),
            sub_option: _self.get_sub_option(),
            len: _self.get_len(),
            status: _self.get_status(),
            data: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableBlockPacket<'p> {
    type T = Block;
    #[inline]
    fn from_packet(&self) -> Block {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Block {
            option: _self.get_option(),
            sub_option: _self.get_sub_option(),
            len: _self.get_len(),
            status: _self.get_status(),
            data: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for BlockPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "BlockPacket {{ option : {:?}, sub_option : {:?}, len : {:?}, status : {:?},  }}",
            _self.get_option(),
            _self.get_sub_option(),
            _self.get_len(),
            _self.get_status()
        )
    }
}
impl<'p> ::std::fmt::Debug for MutableBlockPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "MutableBlockPacket {{ option : {:?}, sub_option : {:?}, len : {:?}, status : {:?},  }}",
            _self.get_option(), _self.get_sub_option(), _self.get_len(), _self
                .get_status()
        )
    }
}
