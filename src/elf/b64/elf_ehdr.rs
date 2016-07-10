use ::std::io::{Read, Seek};

elf_ehdr!();

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
