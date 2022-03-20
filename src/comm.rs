use anyhow::{bail, Result};
use bytes::Bytes;
use std::ops::RangeBounds;

pub fn u16_to_u8s(a: u16) -> [u8; 2] {
    [(a >> 8) as u8, a as u8]
}
pub fn u32_to_u8s(a: u32) -> [u8; 4] {
    [(a >> 24) as u8, (a >> 16) as u8, (a >> 8) as u8, a as u8]
}
pub fn slice_copy_to_vec(vec: &mut Vec<u8>, data: &[u8]) {
    for i in data {
        vec.push(*i);
    }
}
pub fn group_copy_to_vec(vec: &mut Vec<u8>, data: &(u8, u8)) {
    vec.push(data.0);
    vec.push(data.1);
}

pub fn to_u16(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) | b as u16
}

#[derive(Debug)]
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
        if begin >= end {
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
