#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct EthernetPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableEthernetPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> EthernetPacket<'a> {
    /// Constructs a new EthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<EthernetPacket<'p>> {
        if packet.len() >= EthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(EthernetPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new EthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the EthernetPacket will
    /// own its own data and the underlying buffer will be dropped when the EthernetPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<EthernetPacket<'static>> {
        if packet.len() >= EthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(EthernetPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a EthernetPacket to a EthernetPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> EthernetPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        EthernetPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a EthernetPacket to a EthernetPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> EthernetPacket<'a> {
        EthernetPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize {
        14
    }
    /// The size (in bytes) of a Ethernet instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ethernet) -> usize {
        14 + _packet.payload.len()
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &EthernetPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &EthernetPacket) -> u8 {
            let co = 2;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &EthernetPacket) -> u8 {
            let co = 3;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &EthernetPacket) -> u8 {
            let co = 4;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &EthernetPacket) -> u8 {
            let co = 5;
            (_self.packet[co] as u8)
        }
        MacAddr::new(
            get_arg0(&self),
            get_arg1(&self),
            get_arg2(&self),
            get_arg3(&self),
            get_arg4(&self),
            get_arg5(&self),
        )
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &EthernetPacket) -> u8 {
            let co = 7;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &EthernetPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &EthernetPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &EthernetPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &EthernetPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        MacAddr::new(
            get_arg0(&self),
            get_arg1(&self),
            get_arg2(&self),
            get_arg3(&self),
            get_arg4(&self),
            get_arg5(&self),
        )
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
}
impl<'a> MutableEthernetPacket<'a> {
    /// Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableEthernetPacket<'p>> {
        if packet.len() >= MutableEthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableEthernetPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableEthernetPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableEthernetPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableEthernetPacket<'static>> {
        if packet.len() >= MutableEthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableEthernetPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableEthernetPacket to a EthernetPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> EthernetPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        EthernetPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableEthernetPacket to a EthernetPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> EthernetPacket<'a> {
        EthernetPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize {
        14
    }
    /// The size (in bytes) of a Ethernet instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ethernet) -> usize {
        14 + _packet.payload.len()
    }
    /// Populates a EthernetPacket using a Ethernet structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ethernet) {
        let _self = self;
        _self.set_destination(packet.destination);
        _self.set_source(packet.source);
        _self.set_ethertype(packet.ethertype);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableEthernetPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableEthernetPacket) -> u8 {
            let co = 2;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableEthernetPacket) -> u8 {
            let co = 3;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableEthernetPacket) -> u8 {
            let co = 4;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableEthernetPacket) -> u8 {
            let co = 5;
            (_self.packet[co] as u8)
        }
        MacAddr::new(
            get_arg0(&self),
            get_arg1(&self),
            get_arg2(&self),
            get_arg3(&self),
            get_arg4(&self),
            get_arg5(&self),
        )
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableEthernetPacket) -> u8 {
            let co = 7;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableEthernetPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableEthernetPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableEthernetPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableEthernetPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        MacAddr::new(
            get_arg0(&self),
            get_arg1(&self),
            get_arg2(&self),
            get_arg3(&self),
            get_arg4(&self),
            get_arg5(&self),
        )
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
    /// Set the value of the destination field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: MacAddr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 1;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 2;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 3;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 4;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 5;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
    }
    /// Set the value of the source field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: MacAddr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 6;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 7;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 8;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 9;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 10;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 11;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
    }
    /// Set the value of the ethertype field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ethertype(&mut self, val: EtherType) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u16) {
            let co = 12;
            _self.packet[co + 0] = ((val & 0xff00) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        let mut _self = self;
        let current_offset = 14;
        _self.packet[current_offset..current_offset + vals.len()].copy_from_slice(vals);
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for EthernetPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        14
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableEthernetPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        14
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableEthernetPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 14;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableEthernetPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 14;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for EthernetPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 14;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `EthernetPacket`s
pub struct EthernetIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for EthernetIterable<'a> {
    type Item = EthernetPacket<'a>;
    fn next(&mut self) -> Option<EthernetPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = EthernetPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for EthernetPacket<'p> {
    type T = Ethernet;
    #[inline]
    fn from_packet(&self) -> Ethernet {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ethernet {
            destination: _self.get_destination(),
            source: _self.get_source(),
            ethertype: _self.get_ethertype(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableEthernetPacket<'p> {
    type T = Ethernet;
    #[inline]
    fn from_packet(&self) -> Ethernet {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ethernet {
            destination: _self.get_destination(),
            source: _self.get_source(),
            ethertype: _self.get_ethertype(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for EthernetPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "EthernetPacket {{ destination : {:?}, source : {:?}, ethertype : {:?},  }}",
            _self.get_destination(), _self.get_source(), _self.get_ethertype()
        )
    }
}
impl<'p> ::std::fmt::Debug for MutableEthernetPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "MutableEthernetPacket {{ destination : {:?}, source : {:?}, ethertype : {:?},  }}",
            _self.get_destination(), _self.get_source(), _self.get_ethertype()
        )
    }
}
