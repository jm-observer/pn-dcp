use crate::comm::BytesWrap;
use crate::dcp_block::{BlockCommon, BlockIp, BlockResp};
use crate::options::ip::IpBlockInfo;
use crate::pn_dcp::ident_resp::{IdentRespBlock, PacketIdentResp};
use anyhow::{bail, Result};
use bytes::Bytes;
use std::fmt::{Debug, Formatter};
use std::net::Ipv4Addr;

pub mod ip;

// static const value_string pn_dcp_block_info[] = {
// { 0x0000, "RESERVED" },
// /*0x0001 - 0xffff reserved */
// { 0, NULL }
// };
#[derive(Debug, Eq, PartialEq)]
pub enum BlockInfo {
    Reserved,
    UnSupport([u8; 2]),
}
impl BlockInfo {
    pub fn to_u8_array(&self) -> [u8; 2] {
        match self {
            Self::Reserved => [0x00, 0x00],
            Self::UnSupport(a) => a.clone(),
        }
    }
}

impl TryFrom<BytesWrap> for BlockInfo {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> std::result::Result<Self, Self::Error> {
        let val = value.slice(0..=1)?;
        let data = [val.as_ref()[0], val.as_ref()[1]];
        Ok(match data {
            RESERVED => Self::Reserved,
            data => Self::UnSupport(data),
        })
    }
}

const RESERVED: [u8; 2] = [0x00, 0x00];
const USE_TEMPORARY: [u8; 2] = [0x00, 0x00];
const SAVE_PERMANENT: [u8; 2] = [0x00, 0x01];
#[derive(Eq, PartialEq)]
pub enum BlockQualifier {
    UseTemporary,
    SavePermanent,
    UnSupport([u8; 2]),
}
impl BlockQualifier {
    pub fn to_u8_array(&self) -> [u8; 2] {
        match self {
            Self::UnSupport(a) => a.clone(),
            Self::UseTemporary => USE_TEMPORARY,
            Self::SavePermanent => SAVE_PERMANENT,
        }
    }
}
impl Debug for BlockQualifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseTemporary => {
                write!(f, "Use the value temporary")
            }
            Self::SavePermanent => {
                write!(f, "Save the value permanent")
            }
            Self::UnSupport(a) => {
                write!(f, "UnSupport value {:?}", a)
            }
        }
    }
}
impl TryFrom<BytesWrap> for BlockQualifier {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> std::result::Result<Self, Self::Error> {
        let val = value.slice(0..=1)?;
        let a = [val.as_ref()[0], val.as_ref()[1]];
        Ok(match a {
            USE_TEMPORARY => Self::UseTemporary,
            SAVE_PERMANENT => Self::SavePermanent,
            b => Self::UnSupport(b),
        })
    }
}

// impl From<[u8; 2]> for BlockQualifier {
//     fn from(a: [u8; 2]) -> Self {
//         match a {
//             USE_TEMPORARY => Self::UseTemporary,
//             SAVE_PERMANENT => Self::SavePermanent,
//             b => Self::UnSupport(b),
//         }
//     }
// }

trait OptionBuilder {
    fn build(self) -> OptionAndSubValue;
    // fn build_to_ident_resp_default(self, packet: &mut PacketIdentResp) {
    //     packet.append_block(IdentRespBlock::from(BlockCommon::new(self.build())));
    // }
}

// pub struct OptionAndSubValueBuilder;
// impl OptionAndSubValueBuilder {
//     pub fn build_device_options() -> DeviceOptionsBuilder {
//         DeviceOptionsBuilder::default()
//     }
//     // pub fn build_ip_addr_options(
//     //     ip: Ipv4Addr,
//     //     subnetmask: Ipv4Addr,
//     //     gateway: Ipv4Addr,
//     // ) -> IpAddrBuilder {
//     //     IpAddrBuilder(ip, subnetmask, gateway, IpBlockInfo::default())
//     // }
// }
#[derive(Debug, Eq, PartialEq)]
pub struct InnerIpAddr(pub Ipv4Addr, pub Ipv4Addr, pub Ipv4Addr);
impl InnerIpAddr {
    pub fn new(data: BytesWrap) -> Result<Self> {
        let val = data.slice(0..=11)?;
        let val = val.as_ref();
        Ok(Self(
            Ipv4Addr::new(val[0], val[1], val[2], val[3]),
            Ipv4Addr::new(val[4], val[5], val[6], val[7]),
            Ipv4Addr::new(val[8], val[9], val[10], val[11]),
        ))
    }
    pub fn new_by_ipv4(ip: Ipv4Addr, subnetmask: Ipv4Addr, gateway: Ipv4Addr) -> Self {
        Self(ip, subnetmask, gateway)
    }
    pub fn append_value_to_data(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(self.0.octets().as_slice());
        data.extend_from_slice(self.1.octets().as_slice());
        data.extend_from_slice(self.2.octets().as_slice());
    }
    pub fn payload_size(&self) -> usize {
        12
    }
    pub fn to_option(self) -> OptionAndSubValue {
        OptionAndSubValue::IpAddr(self)
    }

    pub fn append_to_ident_resp(self, packet: &mut PacketIdentResp, info: IpBlockInfo) {
        packet.append_block_ip(self, info)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response(pub OptionAndSub, pub BlockError);
impl Response {
    pub fn len(&self) -> usize {
        5
    }
    pub fn payload(&self) -> u16 {
        3
    }
    pub fn append_value_to_data(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(self.0.to_u8_array().as_slice());
        data.push(self.1 as u8);
    }
    pub fn to_option(self) -> OptionAndSubValue {
        OptionAndSubValue::Response(self)
    }
}

impl TryFrom<BytesWrap> for Response {
    type Error = anyhow::Error;
    fn try_from(data: BytesWrap) -> Result<Self, Self::Error> {
        let val = data.slice(0..=2)?;
        let ref_u8 = val.as_ref();
        Ok(Self(
            OptionAndSub::try_from(val.clone())?,
            BlockError::try_from(ref_u8[2])?,
        ))
    }
}

#[derive(Default)]
pub struct DeviceOptionsBuilder(Vec<OptionAndSub>);

impl DeviceOptionsBuilder {
    pub fn append_option(mut self, option: OptionAndSub) -> Self {
        self.0.push(option);
        self
    }
    pub fn build(self) -> OptionAndSubValue {
        OptionAndSubValue::DeviceOptions(self.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum OptionAndSubValue {
    // MarAddr([u8; 6]),
    IpAddr(InnerIpAddr),
    // FullIpSuite(Ipv4Addr, Ipv4Addr, Ipv4Addr, Ipv4Addr),
    ManufacturerSpecific(BytesWrap),
    NameOfStation(BytesWrap),
    DeviceId([u8; 2], [u8; 2]),
    DeviceRole(u8, u8), //DeviceRoleDetails + reserved
    DeviceOptions(Vec<OptionAndSub>),
    // AliasName, //Filter only?
    // StartTransaction,
    // EndTransaction,
    // Signal,
    Response(Response), // not support yet
                        // ResetFactory,
                        // DevicecInitiative,
                        // All,
                        // DHCP(u8),
                        // LLDP(u8),
                        // Other((u8, u8)),
}

impl OptionAndSubValue {
    pub fn append_option_to_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::IpAddr(_) => {
                data.extend_from_slice(OptionAndSub::IpAddr.to_u8_array().as_slice())
            }
            // Self::FullIpSuite(_, _, _, _) => 16,
            Self::ManufacturerSpecific(val) => {
                data.extend_from_slice(OptionAndSub::ManufacturerSpecific.to_u8_array().as_slice())
            }
            Self::NameOfStation(val) => {
                data.extend_from_slice(OptionAndSub::NameOfStation.to_u8_array().as_slice())
            }
            Self::DeviceId(_, _) => {
                data.extend_from_slice(OptionAndSub::DeviceId.to_u8_array().as_slice())
            }
            Self::DeviceRole(_, _) => {
                data.extend_from_slice(OptionAndSub::DeviceRole.to_u8_array().as_slice())
            }
            Self::DeviceOptions(val) => {
                data.extend_from_slice(OptionAndSub::DeviceOptions.to_u8_array().as_slice())
            }
            Self::Response(_) => {
                data.extend_from_slice(OptionAndSub::Response.to_u8_array().as_slice())
            }
        }
    }
    pub fn append_value_to_data(&self, data: &mut Vec<u8>) {
        match self {
            Self::IpAddr(a) => {
                data.extend_from_slice(a.0.octets().as_slice());
                data.extend_from_slice(a.1.octets().as_slice());
                data.extend_from_slice(a.2.octets().as_slice());
            }
            // Self::FullIpSuite(_, _, _, _) => 16,
            Self::ManufacturerSpecific(val) => {
                data.extend_from_slice(val.as_ref());
            }
            Self::NameOfStation(val) => {
                data.extend_from_slice(val.as_ref());
            }
            Self::DeviceId(a, b) => {
                data.extend_from_slice(a.as_ref());
                data.extend_from_slice(b.as_ref());
            }
            Self::DeviceRole(a, b) => {
                data.push(*a);
                data.push(*b);
            }
            Self::DeviceOptions(val) => {
                for option in val {
                    data.extend_from_slice(option.to_u8_array().as_slice());
                }
            }
            Self::Response(a) => {
                a.append_value_to_data(data);
            }
        }
    }
    // data的长度校验，应该等于求出来的值
    pub fn init_by_ty(ty: OptionAndSub, data: BytesWrap) -> Result<Self> {
        Ok(match ty {
            OptionAndSub::IpAddr => {
                let val = data.slice(0..=11)?;
                let val = val.as_ref();
                Self::IpAddr(InnerIpAddr(
                    Ipv4Addr::new(val[0], val[1], val[2], val[3]),
                    Ipv4Addr::new(val[4], val[5], val[6], val[7]),
                    Ipv4Addr::new(val[8], val[9], val[10], val[11]),
                ))
            }
            OptionAndSub::ManufacturerSpecific => Self::ManufacturerSpecific(data),
            OptionAndSub::NameOfStation => Self::NameOfStation(data),
            OptionAndSub::DeviceId => {
                let val = data.slice(0..=3)?;
                let val = val.as_ref();
                Self::DeviceId([val[0], val[1]], [val[2], val[3]])
            }
            OptionAndSub::DeviceRole => {
                let val = data.slice(0..=1)?;
                let val = val.as_ref();
                Self::DeviceRole(val[0], val[1])
            }
            OptionAndSub::DeviceOptions => {
                let mut index = 0;
                let mut options = Vec::new();
                while let Ok(val) = data.slice(index..) {
                    options.push(OptionAndSub::try_from(val)?);
                    index += 2;
                }
                Self::DeviceOptions(options)
            }
            OptionAndSub::Response => {
                let val = data.slice(0..=2)?;
                Self::Response(Response::try_from(val)?)
            }
            option => {
                bail!("todo {:?} not support", option);
            }
        })
    }
    // pub fn init_ip_addr() -> Self {
    //     todo!()
    // }
    // pub fn init_manufacturer_specific(data: Vec<u8>) -> Self {
    //     Self::ManufacturerSpecific(data)
    // }
    pub fn payload_size(&self) -> usize {
        match self {
            Self::IpAddr(_) => 12,
            // Self::FullIpSuite(_, _, _, _) => 16,
            Self::ManufacturerSpecific(val) => val.len(),
            Self::NameOfStation(val) => val.len(),
            Self::DeviceId(_, _) => 4,
            Self::DeviceRole(_, _) => 2,
            Self::DeviceOptions(val) => val.len() * 2,
            Self::Response(_) => 3,
        }
    }

    pub fn append_to_ident_resp_default(self, packet: &mut PacketIdentResp) {
        packet.append_block_common_default(self);
    }
    pub fn append_to_ident_resp(self, packet: &mut PacketIdentResp, info: BlockInfo) {
        packet.append_block_common(self, info);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OptionAndSub {
    MarAddr,
    IpAddr,
    FullIpSuite,
    ManufacturerSpecific,
    NameOfStation,
    DeviceId,
    DeviceRole,
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
    Other(u8, u8),
}

impl TryFrom<BytesWrap> for OptionAndSub {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> std::result::Result<Self, Self::Error> {
        if let Some(a) = value.as_ref().get(0..=1) {
            OptionAndSub::new(a[0], a[1])
        } else {
            bail!("todo")
        }
    }
}

impl OptionAndSub {
    pub fn new(b: u8, c: u8) -> Result<Self> {
        let a = (b, c);
        Ok(match a {
            (1, 1) => Self::MarAddr,
            (1, 2) => Self::IpAddr,
            (1, 3) => Self::FullIpSuite,
            (2, 1) => Self::ManufacturerSpecific,
            (2, 2) => Self::NameOfStation,
            (2, 3) => Self::DeviceId,
            (2, 4) => Self::DeviceRole,
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
            (a, b) => Self::Other(a, b),
            // _ => bail!("todo"),
        })
    }
    pub fn to_u8s(&self) -> (u8, u8) {
        match *self {
            Self::MarAddr => (1, 1),
            Self::IpAddr => (1, 2),
            Self::FullIpSuite => (1, 3),
            Self::ManufacturerSpecific => (2, 1),
            Self::NameOfStation => (2, 2),
            Self::DeviceId => (2, 3),
            Self::DeviceRole => (2, 4),
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
            Self::Other(a, b) => (a, b),
        }
    }
    #[inline]
    pub fn to_u8_array(&self) -> [u8; 2] {
        match *self {
            Self::MarAddr => [1, 1],
            Self::IpAddr => [1, 2],
            Self::FullIpSuite => [1, 3],
            Self::ManufacturerSpecific => [2, 1],
            Self::NameOfStation => [2, 2],
            Self::DeviceId => [2, 3],
            Self::DeviceRole => [2, 4],
            Self::DeviceOptions => [2, 5],
            Self::AliasName => [2, 6],
            Self::StartTransaction => [5, 1],
            Self::EndTransaction => [5, 2],
            Self::Signal => [5, 3],
            Self::Response => [5, 4],
            Self::ResetFactory => [5, 6],
            Self::DevicecInitiative => [6, 1],
            Self::All => [255, 255],
            Self::DHCP(a) => [3, a],
            Self::LLDP(a) => [4, a],
            Self::Other(a, b) => [a, b],
        }
    }
}

// static const value_string pn_dcp_block_error[] = {
// { 0x00, "Ok" },
// { 0x01, "Option unsupp." },
// { 0x02, "Suboption unsupp. or no DataSet avail." },
// { 0x03, "Suboption not set" },
// { 0x04, "Resource Error" },
// { 0x05, "SET not possible by local reasons" },
// { 0x06, "In operation, SET not possible" },
// /* all others reserved */
// { 0, NULL }
// };
//
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum BlockError {
    Ok = 0x00,
    OptionUnsupp = 0x01,
    SuboptionUnsuppOrNoDataSetAvail = 0x02,
    SuboptionNotSet = 0x03,
    ResourceError = 0x04,
    SETNotPossibleByLocalReasons = 0x05,
    InOoperationSETNotPossible = 0x06,
}

impl TryFrom<u8> for BlockError {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::Ok,
            0x01 => Self::OptionUnsupp,
            0x02 => Self::SuboptionUnsuppOrNoDataSetAvail,
            0x03 => Self::SuboptionNotSet,
            0x04 => Self::ResourceError,
            0x05 => Self::SETNotPossibleByLocalReasons,
            0x06 => Self::InOoperationSETNotPossible,
            _ => bail!("todo"),
        })
    }
}
