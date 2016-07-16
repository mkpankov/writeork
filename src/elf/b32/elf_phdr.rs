use ::std::io::{Read, Seek};
use super::super::elf_ehdr::Elf_Ehdr_T;

elf_phdr!();

pub type Elf32_Phdr = Elf_Phdr<Elf32_Half, Elf32_Word, Elf32_Addr, Elf32_Off>;
