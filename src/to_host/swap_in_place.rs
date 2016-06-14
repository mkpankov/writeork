pub trait SwapInPlace {
    fn swap_in_place(&mut self);
}

macro_rules! swap_in_place {
    ( $t:ty, $size:expr ) => {
        impl SwapInPlace for $t {
            fn swap_in_place(&mut self) {
                let size = ::std::mem::size_of::<$t>();
                assert_eq!(size, $size);
                let self_ptr: *mut [u8; $size] = unsafe {
                    ::std::mem::transmute(self)
                };

                for i in 0..size / 2 {
                    unsafe {
                        ::std::mem::swap(&mut (*self_ptr)[i],
                                       &mut (*self_ptr)[size - i - 1])
                    };
                }
            }
        }
    }
}

swap_in_place!(u64, 8);
swap_in_place!(u32, 4);
swap_in_place!(u16, 2);

#[macro_export]
macro_rules! swap_in_place_wrapper {
    ( $wrapper:ty, $t:ty ) => {
        impl ::to_host::swap_in_place::SwapInPlace for $wrapper {
            fn swap_in_place(&mut self) {
                let self_: &mut $t = unsafe {
                    ::std::mem::transmute(self)
                };
                self_.swap_in_place();
            }
        }
    }
}
