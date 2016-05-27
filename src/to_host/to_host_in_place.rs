use to_host::swap_in_place::SwapInPlace;
use super::Endianness;

pub trait ToHostInPlace: SwapInPlace {
    fn to_host_in_place(&mut self, endianness: &Endianness) {
        use super::Endianness::*;

        match *endianness {
            BE => if cfg!(target_endian = "little") {
                SwapInPlace::swap_in_place(self)
            },
            LE => if cfg!(target_endian = "big") {
                SwapInPlace::swap_in_place(self)
            }
        }
    }
}

impl ToHostInPlace for u64 { }

impl ToHostInPlace for u32 { }

impl ToHostInPlace for u16 { }

#[macro_export]
macro_rules! to_host_in_place_wrapper {
    ( $wrapper:ty, $t:ty ) => {
        impl ToHostInPlace for $wrapper {
            fn to_host_in_place(&mut self, endianness: &Endianness) {
                let self_: &mut $t = unsafe {
                    std::mem::transmute(self)
                };
                self_.to_host_in_place(endianness);
            }
        }
    }
}
