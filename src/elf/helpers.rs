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