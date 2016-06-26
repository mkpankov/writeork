use ::std::fmt::{Display, Formatter};
use ::to_host::to_host_copy::ToHostCopy;
use ::to_host::to_host_in_place::ToHostInPlace;
use ::to_host::{Endianness, ToHostInPlaceStruct, ToHostCopyStruct};
use super::super::elf_ident::ElfIdent;
use super::super::elf_ident_named::ElfIdentNamed;
use super::super::elf_ehdr_type::ElfEhdrType;
use super::super::elf_ehdr_machine::ElfEhdrMachine;
use super::primitive::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf_Ehdr {
    e_ident: ElfIdent,
    e_type: ElfEhdrType,
    e_machine: ElfEhdrMachine,
    e_version: Elf32_Word,
    e_entry: Elf32_Addr,
    e_phoff: Elf32_Off,
    e_shoff: Elf32_Off,
    e_flags: Elf32_Word,
    e_ehsize: Elf32_Half,
    e_phentsize: Elf32_Half,
    e_phnum: Elf32_Half,
    e_shentsize: Elf32_Half,
    e_shnum: Elf32_Half,
    e_shstrndx: Elf32_Half
}

impl Elf_Ehdr {
    fn from_slice(buffer: &[u8]) -> Result<&Elf_Ehdr, ()> {
        let proper_magic = &[0x7f, b'E', b'L', b'F'];
        let magic_ptr: *const [u8; 4] = unsafe {
            ::std::mem::transmute(buffer.as_ptr())
        };
        let magic = unsafe { &*magic_ptr };
        if proper_magic != magic {
            return Err(())
        }

        let ehdr_ptr: *const Elf_Ehdr = unsafe {
            ::std::mem::transmute(buffer.as_ptr())
        };
        let ehdr: &Elf_Ehdr = unsafe { &*ehdr_ptr };

        Ok(ehdr)
    }
}

impl Display for Elf_Ehdr {
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

impl Elf_Ehdr {
    pub fn get_phentsize(&self) -> u16 {
        self.e_phentsize
    }
    pub fn get_phnum(&self) -> u16 {
        self.e_phnum
    }
    pub fn get_phoff(&self) -> u32 {
        self.e_phoff
    }
    pub fn get_entry(&self) -> u32 {
        self.e_entry
    }
    pub fn get_type(&self) -> ElfEhdrType {
        self.e_type
    }
    pub fn get_machine(&self) -> ElfEhdrMachine {
        self.e_machine
    }
    pub fn get_version(&self) -> u32 {
        self.e_version
    }
    pub fn get_shoff(&self) -> u32 {
        self.e_shoff
    }
    pub fn get_flags(&self) -> u32 {
        self.e_flags
    }
    pub fn get_ehsize(&self) -> u16 {
        self.e_ehsize
    }
    pub fn get_shentsize(&self) -> u16 {
        self.e_shentsize
    }
    pub fn get_shnum(&self) -> u16 {
        self.e_shnum
    }
    pub fn get_shstrndx(&self) -> u16 {
        self.e_shstrndx
    }

    pub fn get_endianness(&self) -> Endianness {
        let ehdr_ptr: *mut Elf_Ehdr = unsafe {
            ::std::mem::transmute(self)
        };
        let ehdr: &mut Elf_Ehdr = unsafe { &mut *ehdr_ptr };
        let ehdr_ident: &ElfIdentNamed = unsafe {
            ::std::mem::transmute(&ehdr.e_ident)
        };

        ehdr_ident.get_endianness()
    }
}


impl ToHostInPlaceStruct for Elf_Ehdr {
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

impl ToHostCopyStruct for Elf_Ehdr {
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
