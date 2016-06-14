use to_host::swap_copy::SwapCopy;
use super::Endianness;

pub trait ToHostCopy: SwapCopy + Sized + Copy {
    fn to_host_copy(&self, endianness: &Endianness) -> Self {
        use super::Endianness::*;

        match *endianness {
            BE => if cfg!(target_endian = "little") {
                SwapCopy::swap_copy(self)
            } else {
                *self
            },
            LE => if cfg!(target_endian = "big") {
                SwapCopy::swap_copy(self)
            } else {
                *self
            }
        }
    }
}

impl ToHostCopy for u64 { }

impl ToHostCopy for u32 { }

impl ToHostCopy for u16 { }

#[macro_export]
macro_rules! to_host_copy_wrapper {
    ( $wrapper:ty, $t:ty ) => {
        impl ::to_host::to_host_copy::ToHostCopy for $wrapper {
            fn to_host_copy(&self, endianness: &::to_host::Endianness) -> Self {
                let self_: &$t = unsafe {
                    ::std::mem::transmute(self)
                };
                let result_ = self_.to_host_copy(endianness);
                unsafe {
                    ::std::mem::transmute(result_)
                }
            }
        }
    }
}
