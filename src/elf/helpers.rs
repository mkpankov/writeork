#[macro_export]
macro_rules! read_ehdr {
    () => {
        impl<H, W, A, O> Elf_Ehdr<H, W, A, O> {
            #[allow(dead_code)]
            fn convert_byte_vec_to_ehdr_box(
                mut v: Vec<u8>)
                -> Result<Box<Self>, ()>
            {
                let ehdr_size = ::std::mem::size_of::<Self>();

                assert_eq!(ehdr_size as usize, v.len());
                let bytes_ptr: *mut u8 = v.as_mut_ptr();
                ::std::mem::forget(v);

                let proper_magic = &[0x7f, b'E', b'L', b'F'];
                let magic_ptr: *const [u8; 4] = unsafe {
                    ::std::mem::transmute(bytes_ptr)
                };
                let magic = unsafe { &*magic_ptr };
                if proper_magic != magic {
                    return Err(())
                }

                let ehdr_ptr: *mut Self = unsafe {
                    ::std::mem::transmute(bytes_ptr)
                };
                let ehdr_box: Box<Self> = unsafe {
                    Box::from_raw(ehdr_ptr)
                };
                Ok(ehdr_box)
            }

            #[allow(dead_code)]
            pub fn read_ehdr<R: Read + Seek>(
                reader: &mut R)
                -> Box<Self>
            {
                use std::io::SeekFrom;

                let ehdr_size = ::std::mem::size_of::<Self>();
                let ehdr_offset = 0;

                let mut b = Vec::<u8>::with_capacity(ehdr_size as usize);
                reader.seek(SeekFrom::Start(ehdr_offset)).unwrap();
                reader.take(ehdr_size as u64).read_to_end(&mut b).unwrap();

                Self::convert_byte_vec_to_ehdr_box(b).unwrap()
            }
        }
    }
}

#[macro_export]
macro_rules! read_phdrs {
    ($phdr:ty, $ehdr:ty) => {
        #[allow(dead_code)]
        fn convert_byte_vec_to_phdrs_vec(
            v: Vec<u8>, phdr_num: u16, phdr_size: u16) 
            -> Vec<$phdr> {
            assert_eq!(phdr_num as usize * phdr_size as usize, v.len());
            let mut r: Vec<$phdr> = unsafe {
                ::std::mem::transmute(v)
            };
            unsafe {
                r.set_len(phdr_num as usize);
            }
            r
        }

        #[allow(dead_code)]
        pub fn read_phdrs<R: Read + Seek>(
            ehdr: &$ehdr, reader: &mut R)
            -> Vec<$phdr>
        {
            use std::io::SeekFrom;

            let phdr_size = ehdr.get_phentsize() * ehdr.get_phnum();
            let phdr_offset = ehdr.get_phoff();
            let phdr_num = ehdr.get_phnum();

            let mut b = Vec::<u8>::with_capacity(phdr_size as usize * phdr_num as usize);
            reader.seek(SeekFrom::Start(phdr_offset as u64)).unwrap();
            reader.take(phdr_size as u64 * phdr_num as u64).read_to_end(&mut b).unwrap();

            convert_byte_vec_to_phdrs_vec(b, phdr_num, phdr_size)
        }
    }
}