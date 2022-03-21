use crate::comm::{to_u16, BytesWrap};
use crate::options::ip::IpBlockInfo;
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
#[derive(Debug)]
pub enum BlockInfo {
    Reserved,
    UnSupport([u8; 2]),
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

pub enum BlockQualifier {
    UseTemporary,
    SavePermanent,
    UnSupport([u8; 2]),
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

#[derive(Debug)]
pub enum OptionAndSubValue {
    // MarAddr([u8; 6]),
    IpAddr(Ipv4Addr, Ipv4Addr, Ipv4Addr),
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
    Response(OptionAndSub, BlockError), // not support yet
                                        // ResetFactory,
                                        // DevicecInitiative,
                                        // All,
                                        // DHCP(u8),
                                        // LLDP(u8),
                                        // Other((u8, u8)),
}

impl OptionAndSubValue {
    // data的长度校验，应该等于求出来的值
    pub fn init_by_ty(ty: OptionAndSub, data: BytesWrap) -> Result<Self> {
        Ok(match ty {
            OptionAndSub::IpAddr => {
                let val = data.slice(0..=11)?;
                let val = val.as_ref();
                Self::IpAddr(
                    Ipv4Addr::new(val[0], val[1], val[2], val[3]),
                    Ipv4Addr::new(val[4], val[5], val[6], val[7]),
                    Ipv4Addr::new(val[8], val[9], val[10], val[11]),
                )
            }
            // OptionAndSub::FullIpSuite => {
            //     let val = data.slice(0..=15)?;
            //     let val = val.as_ref();
            //     Self::FullIpSuite(
            //         Ipv4Addr::new(val[0], val[1], val[2], val[3]),
            //         Ipv4Addr::new(val[4], val[5], val[6], val[7]),
            //         Ipv4Addr::new(val[8], val[9], val[10], val[11]),
            //         Ipv4Addr::new(val[12], val[13], val[14], val[15]),
            //     )
            // }
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
                let ref_u8 = val.as_ref();
                Self::Response(
                    OptionAndSub::try_from(val.clone())?,
                    BlockError::try_from(ref_u8[2])?,
                )
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
            Self::IpAddr(_, _, _) => 12,
            // Self::FullIpSuite(_, _, _, _) => 16,
            Self::ManufacturerSpecific(val) => val.len(),
            Self::NameOfStation(val) => val.len(),
            Self::DeviceId(_, _) => 4,
            Self::DeviceRole(_, _) => 2,
            Self::DeviceOptions(val) => val.len() * 2,
            Self::Response(_, _) => 3,
        }
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
#[derive(Debug)]
pub enum BlockError {
    Ok,
    OptionUnsupp,
    SuboptionUnsuppOrNoDataSetAvail,
    SuboptionNotSet,
    ResourceError,
    SETNotPossibleByLocalReasons,
    InOoperationSETNotPossible,
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
