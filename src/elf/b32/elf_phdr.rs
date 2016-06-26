use ::to_host::Endianness;
use ::to_host::to_host_copy::ToHostCopy;
use ::to_host::ToHostCopyStruct;
use super::super::elf_phdr_type::ElfPhdrType;
use super::super::elf_phdr_flags::ElfPhdrFlags;
use super::primitive::*;

#[derive(Debug)]
#[repr(C)]
pub struct Elf_Phdr {
    p_type: Elf32_Word,
    p_flags: Elf32_Word,
    p_offset: Elf32_Off,
    p_vaddr: Elf32_Addr,
    p_paddr: Elf32_Addr,
    p_filesz: Elf32_Word,
    p_memsz: Elf32_Word,
    p_align: Elf32_Word,
}

impl Elf_Phdr {
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

impl Elf_Phdr {
    #[allow(dead_code)]
    fn from_slice(buffer: &[u8]) -> &Elf_Phdr {
        let phdr_ptr: *const Elf_Phdr = unsafe {
            ::std::mem::transmute(buffer.as_ptr())
        };
        let phdr: &Elf_Phdr = unsafe { &*phdr_ptr };

        phdr
    }
}

impl ToHostCopyStruct for Elf_Phdr {
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
        }
    }
}
