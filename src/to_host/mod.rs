pub mod swap_copy;
pub mod swap_in_place;
pub mod to_host_in_place;
pub mod to_host_copy;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Endianness {
    LE,
    BE,
}

pub trait ToHost {
    fn to_host_in_place(&mut self, endianness: &Endianness);
    fn to_host_copy(&self, endianness: &Endianness) -> Self;
}
