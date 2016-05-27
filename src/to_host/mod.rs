#[macro_use]
pub mod swap_copy;
#[macro_use]
pub mod swap_in_place;
#[macro_use]
pub mod to_host_in_place;
#[macro_use]
pub mod to_host_copy;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Endianness {
    LE,
    BE,
}

pub trait ToHostInPlaceStruct {
    fn to_host_in_place(&mut self, endianness: &Endianness);
}

pub trait ToHostCopyStruct {
    fn to_host_copy(&self, endianness: &Endianness) -> Self;
}
