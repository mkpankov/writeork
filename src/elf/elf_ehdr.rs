#[macro_export]
macro_rules! elf_ehdr {
    () => {
        use ::std::fmt::{Display, Formatter};
        use ::to_host::to_host_copy::ToHostCopy;
        use ::to_host::to_host_in_place::ToHostInPlace;
        use ::to_host::{Endianness, ToHostInPlaceStruct, ToHostCopyStruct};
        use super::super::elf_ident::ElfIdent;
        use super::super::elf_ident_named::ElfIdentNamed;
        use super::super::elf_ehdr_type::ElfEhdrType;
        use super::super::elf_ehdr_machine::ElfEhdrMachine;

        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct Elf_Ehdr<H, W, A, O> {
            e_ident: ElfIdent,
            e_type: ElfEhdrType,
            e_machine: ElfEhdrMachine,
            e_version: W,
            e_entry: A,
            e_phoff: O,
            e_shoff: O,
            e_flags: W,
            e_ehsize: H,
            e_phentsize: H,
            e_phnum: H,
            e_shentsize: H,
            e_shnum: H,
            e_shstrndx: H
        }

        impl<H, W, A, O> Display
            for Elf_Ehdr<H, W, A, O>
            where
                H: ToHostCopy + ::std::fmt::LowerHex + ::std::fmt::Display,
                W: ToHostCopy + ::std::fmt::LowerHex,
                A: ToHostCopy + ::std::fmt::LowerHex,
                O: ToHostCopy + ::std::fmt::LowerHex + ::std::fmt::Display,
        {
            fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
                let ehdr_ident: &ElfIdentNamed = unsafe {
                    ::std::mem::transmute(&self.e_ident)
                };

                let e = self.get_endianness();

                write!(
                    fmt,
                    concat!(
                        "ELF Header:\n",
                        "  Magic:   {}\n",
                        "{}",
                        "  Type:                              {}\n",
                        "  Machine:                           {}\n",
                        "  Version:                           {:#x}\n",
                        "  Entry point address:               {:#x}\n",
                        "  Start of program headers:          {} (bytes into file)\n",
                        "  Start of section headers:          {} (bytes into file)\n",
                        "  Flags:                             {:#x}\n",
                        "  Size of this header:               {} (bytes)\n",
                        "  Size of program headers:           {} (bytes)\n",
                        "  Number of program headers:         {}\n",
                        "  Size of section headers:           {} (bytes)\n",
                        "  Number of section headers:         {}\n",
                        "  Section header string table index: {}\n",
                    ),
                    self.e_ident,
                    ehdr_ident,
                    self.e_type.to_host_copy(&e),
                    self.e_machine.to_host_copy(&e),
                    self.e_version.to_host_copy(&e),
                    self.e_entry.to_host_copy(&e),
                    self.e_phoff.to_host_copy(&e),
                    self.e_shoff.to_host_copy(&e),
                    self.e_flags.to_host_copy(&e),
                    self.e_ehsize.to_host_copy(&e),
                    self.e_phentsize.to_host_copy(&e),
                    self.e_phnum.to_host_copy(&e),
                    self.e_shentsize.to_host_copy(&e),
                    self.e_shnum.to_host_copy(&e),
                    self.e_shstrndx.to_host_copy(&e))
            }
        }

        impl<H, W, A, O> Elf_Ehdr<H, W, A, O> 
        where
            H: Copy,
            W: Copy,
            A: Copy,
            O: Copy,
        {
            #[allow(dead_code)]
            fn validate_magic(buffer: &[u8]) -> Result<(), ()> 
            {
                let proper_magic = &[0x7f, b'E', b'L', b'F'];
                if buffer.len() < 4 {
                    return Err(())
                }
                let magic = &buffer[..4];
                if magic == proper_magic {
                    Ok(())
                } else {
                    Err(())
                }
            }

            #[allow(dead_code)]
            fn from_slice(buffer: &[u8]) -> Result<&Elf_Ehdr<H, W, A, O>, ()> 
            {
                try!(Self::validate_magic(buffer));
                
                let ehdr_ptr: *const Elf_Ehdr<H, W, A, O> = unsafe {
                    ::std::mem::transmute(buffer.as_ptr())
                };
                let ehdr: &Elf_Ehdr<H, W, A, O> = unsafe { &*ehdr_ptr };

                Ok(ehdr)
            }
            #[allow(dead_code)]
            pub fn get_phentsize(&self) -> H {
                self.e_phentsize
            }
            #[allow(dead_code)]
            pub fn get_phnum(&self) -> H {
                self.e_phnum
            }
            #[allow(dead_code)]
            pub fn get_phoff(&self) -> O {
                self.e_phoff
            }
            #[allow(dead_code)]
            pub fn get_entry(&self) -> A {
                self.e_entry
            }
            #[allow(dead_code)]
            pub fn get_ident(&self) -> ElfIdentNamed {
                unsafe { 
                    ::std::mem::transmute(self.e_ident)
                }
            }
            #[allow(dead_code)]
            pub fn get_type(&self) -> ElfEhdrType {
                self.e_type
            }
            #[allow(dead_code)]
            pub fn get_machine(&self) -> ElfEhdrMachine {
                self.e_machine
            }
            #[allow(dead_code)]
            pub fn get_version(&self) -> W {
                self.e_version
            }
            #[allow(dead_code)]
            pub fn get_shoff(&self) -> O {
                self.e_shoff
            }
            #[allow(dead_code)]
            pub fn get_flags(&self) -> W {
                self.e_flags
            }
            #[allow(dead_code)]
            pub fn get_ehsize(&self) -> H {
                self.e_ehsize
            }
            #[allow(dead_code)]
            pub fn get_shentsize(&self) -> H {
                self.e_shentsize
            }
            #[allow(dead_code)]
            pub fn get_shnum(&self) -> H {
                self.e_shnum
            }
            #[allow(dead_code)]
            pub fn get_shstrndx(&self) -> H {
                self.e_shstrndx
            }

            #[allow(dead_code)]
            pub fn get_endianness(&self) -> Endianness {
                let ehdr_ptr: *mut Elf_Ehdr<H, W, A, O> = unsafe {
                    ::std::mem::transmute(self)
                };
                let ehdr: &mut Elf_Ehdr<H, W, A, O> = unsafe { &mut *ehdr_ptr };
                let ehdr_ident: &ElfIdentNamed = unsafe {
                    ::std::mem::transmute(&ehdr.e_ident)
                };

                ehdr_ident.get_endianness()
            }

            #[allow(dead_code)]
            fn convert_byte_vec_to_ehdr_box(
                mut v: Vec<u8>)
                -> Result<Box<Self>, ()>
            {
                let ehdr_size = ::std::mem::size_of::<Self>();

                assert_eq!(ehdr_size as usize, v.len());
                try!(Self::validate_magic(&v));

                let bytes_ptr: *mut u8 = v.as_mut_ptr();
                ::std::mem::forget(v);

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

        impl<H, W, A, O> ToHostInPlaceStruct for Elf_Ehdr<H, W, A, O> 
        where
            H: ToHostInPlace, 
            W: ToHostInPlace,
            A: ToHostInPlace,
            O: ToHostInPlace,
        {
            fn to_host_in_place(&mut self, endianness: &Endianness) {
                let e = endianness;
                self.e_type.to_host_in_place(e);
                self.e_machine.to_host_in_place(e);
                self.e_version.to_host_in_place(e);
                self.e_entry.to_host_in_place(e);
                self.e_phoff.to_host_in_place(e);
                self.e_shoff.to_host_in_place(e);
                self.e_flags.to_host_in_place(e);
                self.e_ehsize.to_host_in_place(e);
                self.e_phentsize.to_host_in_place(e);
                self.e_phnum.to_host_in_place(e);
                self.e_shentsize.to_host_in_place(e);
                self.e_shnum.to_host_in_place(e);
                self.e_shstrndx.to_host_in_place(e);
            }
        }

        impl<H, W, A, O> ToHostCopyStruct for Elf_Ehdr<H, W, A, O> 
            where
                H: ToHostCopy, 
                W: ToHostCopy,
                A: ToHostCopy,
                O: ToHostCopy,
        {
            fn to_host_copy(&self, endianness: &Endianness) -> Self {
                let e = endianness;
                Elf_Ehdr {
                    e_ident: self.e_ident,
                    e_type: self.e_type.to_host_copy(e),
                    e_machine: self.e_machine.to_host_copy(e),
                    e_version: self.e_version.to_host_copy(e),
                    e_entry: self.e_entry.to_host_copy(e),
                    e_phoff: self.e_phoff.to_host_copy(e),
                    e_shoff: self.e_shoff.to_host_copy(e),
                    e_flags: self.e_flags.to_host_copy(e),
                    e_ehsize: self.e_ehsize.to_host_copy(e),
                    e_phentsize: self.e_phentsize.to_host_copy(e),
                    e_phnum: self.e_phnum.to_host_copy(e),
                    e_shentsize: self.e_shentsize.to_host_copy(e),
                    e_shnum: self.e_shnum.to_host_copy(e),
                    e_shstrndx: self.e_shstrndx.to_host_copy(e),
                }
            }
        }

        impl<H, W, A, O> Elf_Ehdr_T<H, O> for Elf_Ehdr<H, W, A, O>
            where
                H: ::num::PrimInt,
                O: ::num::PrimInt,
        {
            fn get_phentsize(&self) -> H
            {
                self.e_phentsize
            }

            fn get_phnum(&self) -> H
            {
                self.e_phnum
            }
            
            fn get_phoff(&self) -> O 
            {
                self.e_phoff
            }    
        }
    }
}

pub trait Elf_Ehdr_T<H, O> 
    where
        H: ::num::PrimInt,
        O: ::num::PrimInt,
{
    fn get_phentsize(&self) -> H;
    fn get_phnum(&self) -> H;
    fn get_phoff(&self) -> O;
}
