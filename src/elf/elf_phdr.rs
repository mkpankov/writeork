#[macro_export]
macro_rules! elf_phdr {
    () => {
        use ::to_host::Endianness;
        use ::to_host::to_host_copy::ToHostCopy;
        use ::to_host::ToHostCopyStruct;
        use super::super::elf_phdr_type::ElfPhdrType;
        use super::super::elf_phdr_flags::ElfPhdrFlags;
        use super::primitive::*;

        #[derive(Debug)]
        #[repr(C)]
        pub struct Elf_Phdr<H, X, A, O> {
            p_type: ElfPhdrType,
            p_flags: ElfPhdrFlags,
            p_offset: O,
            p_vaddr: A,
            p_paddr: A,
            p_filesz: X,
            p_memsz: X,
            p_align: X,
            _half: ::std::marker::PhantomData<H>,
        }

        impl<H, X, A, O> Elf_Phdr<H, X, A, O>
            where
                X: ToHostCopy + ::std::fmt::LowerHex,
                A: ToHostCopy + ::std::fmt::LowerHex,
                O: ToHostCopy + ::std::fmt::LowerHex, 
        {
            #[allow(dead_code)]
            pub fn print_with_endianness(&self, e: &Endianness) {
                let p_type: ElfPhdrType = unsafe {
                    ::std::mem::transmute(self.p_type.to_host_copy(e))
                };
                let p_flags: ElfPhdrFlags = unsafe {
                    ::std::mem::transmute(self.p_flags.to_host_copy(e))
                };

                print!(
                    concat!(
                        "{: <15}",
                        "{:#08x} ",
                        "{:#018x} ",
                        "{:#018x} ",
                        "{:#08x} ",
                        "{:#08x} ",
                        "{:<3} ",
                        "{:#x}",
                    ),
                    p_type,
                    self.p_offset.to_host_copy(e),
                    self.p_vaddr.to_host_copy(e),
                    self.p_paddr.to_host_copy(e),
                    self.p_filesz.to_host_copy(e),
                    self.p_memsz.to_host_copy(e),
                    p_flags,
                    self.p_align.to_host_copy(e),
                );
            }
        }

        impl<H, X, A, O> Elf_Phdr<H, X, A, O> {
            #[allow(dead_code)]
            fn from_slice(buffer: &[u8]) -> &Elf_Phdr<H, X, A, O> {
                let phdr_ptr: *const Elf_Phdr<H, X, A, O> = unsafe {
                    ::std::mem::transmute(buffer.as_ptr())
                };
                let phdr: &Elf_Phdr<H, X, A, O> = unsafe { &*phdr_ptr };

                phdr
            }
        }

        impl<H, X, A, O> ToHostCopyStruct for Elf_Phdr<H, X, A, O>
            where
                X: ToHostCopy,
                A: ToHostCopy,
                O: ToHostCopy, 
        {
            fn to_host_copy(&self, endianness: &Endianness) -> Self {
                let e = endianness;
                Elf_Phdr {
                    p_type: self.p_type.to_host_copy(e),
                    p_flags: self.p_flags.to_host_copy(e),
                    p_offset: self.p_offset.to_host_copy(e),
                    p_vaddr: self.p_vaddr.to_host_copy(e),
                    p_paddr: self.p_paddr.to_host_copy(e),
                    p_filesz: self.p_filesz.to_host_copy(e),
                    p_memsz: self.p_memsz.to_host_copy(e),
                    p_align: self.p_align.to_host_copy(e),
                    _half: ::std::marker::PhantomData {},
                }
            }
        }

        impl<H, X, A, O> Elf_Phdr<H, X, A, O> {
            #[allow(dead_code)]
            fn convert_byte_vec_to_phdrs_vec(
                v: Vec<u8>, phdr_num: u16, phdr_size: u16) 
                -> Vec<Self> {
                assert_eq!(phdr_num as usize * phdr_size as usize, v.len());
                let mut r: Vec<Self> = unsafe {
                    ::std::mem::transmute(v)
                };
                unsafe {
                    r.set_len(phdr_num as usize);
                }
                r
            }

            #[allow(dead_code)]
            pub fn read_phdrs<R: Read + Seek, E: Elf_Ehdr_T<H, O>>(
                ehdr: &E, reader: &mut R)
                -> Vec<Self>
                where
                    H: ::num::PrimInt + ::num::ToPrimitive,
                    O: ::num::PrimInt + ::num::ToPrimitive,
            {
                use std::io::SeekFrom;

                let phdr_size = ehdr.get_phentsize() * ehdr.get_phnum();
                let phdr_offset = ehdr.get_phoff();
                let phdr_num = ehdr.get_phnum();

                let mut b = Vec::<u8>::with_capacity(
                    phdr_size.to_usize().unwrap()
                  * phdr_num.to_usize().unwrap());
                reader
                    .seek(
                        SeekFrom::Start(
                            phdr_offset.to_u64().unwrap()))
                    .unwrap();
                reader
                    .take(
                        phdr_size.to_u64().unwrap()
                      * phdr_num.to_u64().unwrap())
                    .read_to_end(&mut b)
                    .unwrap();

                Self::convert_byte_vec_to_phdrs_vec(
                    b, phdr_num.to_u16().unwrap(), phdr_size.to_u16().unwrap())
            }
        }
    }
}
