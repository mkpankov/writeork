pub trait SwapCopy {
    fn swap_copy(&self) -> Self;
}

macro_rules! swap_copy {
    ( $t:ty, $size:expr ) => {
        impl SwapCopy for $t {
            fn swap_copy(&self) -> Self {
                let result: $t = *self;
                let size = ::std::mem::size_of::<$t>();
                assert_eq!(size, $size);
                let self_ptr: *mut [u8; $size] = unsafe {
                    ::std::mem::transmute(&result)
                };

                for i in 0..size / 2 {
                    unsafe {
                        ::std::mem::swap(&mut (*self_ptr)[i],
                                       &mut (*self_ptr)[size - i - 1])
                    };
                }
                result
            }
        }
    }
}

swap_copy!(u64, 8);
swap_copy!(u32, 4);
swap_copy!(u16, 2);

#[macro_export]
macro_rules! swap_copy_wrapper {
    ( $wrapper:ty, $t:ty ) => {
        impl ::to_host::swap_copy::SwapCopy for $wrapper {
            fn swap_copy(&self) -> Self {
                let self_: &$t = unsafe {
                    ::std::mem::transmute(self)
                };
                let result_ = self_.swap_copy();
                unsafe {
                    ::std::mem::transmute(result_)
                }
            }
        }
    }
}
