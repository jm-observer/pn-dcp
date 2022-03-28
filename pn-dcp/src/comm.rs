use anyhow::{bail, Result};
use bytes::Bytes;
use std::ops::RangeBounds;

use pnet::packet::ethernet::EtherType;

pub const PROFINET_ETHER_TYPE: EtherType = EtherType(0x8892);

#[derive(Debug, Eq, PartialEq)]
pub struct BytesWrap(Bytes);

impl From<Bytes> for BytesWrap {
    fn from(a: Bytes) -> Self {
        Self(a)
    }
}
impl From<Vec<u8>> for BytesWrap {
    fn from(a: Vec<u8>) -> Self {
        Self(a.into())
    }
}
impl From<&[u8]> for BytesWrap {
    fn from(a: &[u8]) -> Self {
        Self(a.to_vec().into())
    }
}

impl Clone for BytesWrap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl AsRef<[u8]> for BytesWrap {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl BytesWrap {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn split_off(&mut self, at: usize) -> Result<Self> {
        if at > self.len() {
            bail!("split_off out of bounds: {:?} <= {:?}", at, self.len(),)
        }
        Ok(Self(self.0.split_off(at)))
    }
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Result<Self> {
        use core::ops::Bound;

        let len = self.len();

        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        if begin > end {
            bail!(
                "range start must not be greater than end: {:?} <= {:?}",
                begin,
                end
            );
        }
        if end > len {
            bail!("range end out of bounds: {:?} <= {:?}", end, len);
        }
        let bytes = self.0.slice(range);
        Ok(Self(bytes))
    }
}
