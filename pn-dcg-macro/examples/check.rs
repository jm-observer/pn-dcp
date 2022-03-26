use pn_dcg_macro::derefmut;
use std::ops::{Deref, DerefMut};

pub fn main() {}

#[derefmut(0)]
struct A(usize);

#[derefmut(b)]
struct B {
    b: u32,
}
