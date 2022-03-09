pub fn u16_to_u8s(a: u16) -> [u8; 2] {
    [(a >> 8) as u8, a as u8]
}
pub fn u32_to_u8s(a: u32) -> [u8; 4] {
    [(a >> 24) as u8, (a >> 16) as u8, (a >> 8) as u8, a as u8]
}
