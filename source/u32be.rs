#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct U32Packet<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableU32Packet<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> U32Packet<'a> {
    /// Constructs a new U32Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<U32Packet<'p>> {
        if packet.len() >= U32Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(U32Packet {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new U32Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the U32Packet will
    /// own its own data and the underlying buffer will be dropped when the U32Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<U32Packet<'static>> {
        if packet.len() >= U32Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(U32Packet {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a U32Packet to a U32Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> U32Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        U32Packet {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a U32Packet to a U32Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> U32Packet<'a> {
        U32Packet {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize {
        4
    }
    /// The size (in bytes) of a U32 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &U32) -> usize {
        4 + _packet.block.len()
    }
    /// Get the xid field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_xid(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
}
impl<'a> MutableU32Packet<'a> {
    /// Constructs a new MutableU32Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableU32Packet<'p>> {
        if packet.len() >= MutableU32Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableU32Packet {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableU32Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableU32Packet will
    /// own its own data and the underlying buffer will be dropped when the MutableU32Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableU32Packet<'static>> {
        if packet.len() >= MutableU32Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableU32Packet {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableU32Packet to a U32Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> U32Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        U32Packet {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableU32Packet to a U32Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> U32Packet<'a> {
        U32Packet {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize {
        4
    }
    /// The size (in bytes) of a U32 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &U32) -> usize {
        4 + _packet.block.len()
    }
    /// Populates a U32Packet using a U32 structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &U32) {
        let _self = self;
        _self.set_xid(packet.xid);
        _self.set_block(&packet.block);
    }
    /// Get the xid field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_xid(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Set the xid field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_xid(&mut self, val: u32be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((val & 0xff000000) >> 24) as u8;
        _self.packet[co + 1] = ((val & 0xff0000) >> 16) as u8;
        _self.packet[co + 2] = ((val & 0xff00) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the value of the block field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_block(&mut self, vals: &[u8]) {
        let mut _self = self;
        let current_offset = 4;
        _self.packet[current_offset..current_offset + vals.len()].copy_from_slice(vals);
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for U32Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableU32Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableU32Packet<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableU32Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for U32Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `U32Packet`s
pub struct U32Iterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for U32Iterable<'a> {
    type Item = U32Packet<'a>;
    fn next(&mut self) -> Option<U32Packet<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = U32Packet::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for U32Packet<'p> {
    type T = U32;
    #[inline]
    fn from_packet(&self) -> U32 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        U32 {
            xid: _self.get_xid(),
            block: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableU32Packet<'p> {
    type T = U32;
    #[inline]
    fn from_packet(&self) -> U32 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        U32 {
            xid: _self.get_xid(),
            block: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for U32Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt, "U32Packet {{ xid : {:?},  }}", _self.get_xid())
    }
}
impl<'p> ::std::fmt::Debug for MutableU32Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt, "MutableU32Packet {{ xid : {:?},  }}", _self.get_xid())
    }
}