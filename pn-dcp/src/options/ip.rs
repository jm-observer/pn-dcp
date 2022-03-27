// static const value_string pn_dcp_suboption_ip_block_info[] = {
// { 0x0000, "IP not set" },
// { 0x0001, "IP set" },
// { 0x0002, "IP set by DHCP" },
// { 0x0080, "IP not set (address conflict detected)" },
// { 0x0081, "IP set (address conflict detected)" },
// { 0x0082, "IP set by DHCP (address conflict detected)" },
// /*0x0003 - 0xffff reserved */
// { 0, NULL }
// };

use crate::comm::BytesWrap;
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IpBlockInfo {
    IpNotSet,
    IpSet,
    IpSetByDhcp,
    IpNotSetConflict,
    IpSetConflict,
    IpSetByDhcpConflict,
    UnSupport([u8; 2]),
}

impl Default for IpBlockInfo {
    fn default() -> Self {
        Self::IpSet
    }
}

impl IpBlockInfo {
    pub fn to_u8_array(&self) -> [u8; 2] {
        match self {
            Self::IpNotSet => [0x00, 0x00],
            Self::IpSet => [0x00, 0x01],
            Self::IpSetByDhcp => [0x00, 0x02],
            Self::IpNotSetConflict => [0x00, 0x80],
            Self::IpSetConflict => [0x00, 0x81],
            Self::IpSetByDhcpConflict => [0x00, 0x82],
            Self::UnSupport(data) => data.clone(),
        }
    }
}

impl TryFrom<BytesWrap> for IpBlockInfo {
    type Error = anyhow::Error;
    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let val = value.slice(0..=1)?;
        let data = [val.as_ref()[0], val.as_ref()[1]];
        Ok(match data {
            [0x00, 0x00] => Self::IpNotSet,
            [0x00, 0x01] => Self::IpSet,
            [0x00, 0x02] => Self::IpSetByDhcp,
            [0x00, 0x80] => Self::IpNotSetConflict,
            [0x00, 0x81] => Self::IpSetConflict,
            [0x00, 0x82] => Self::IpSetByDhcpConflict,
            data => Self::UnSupport(data),
        })
    }
}
