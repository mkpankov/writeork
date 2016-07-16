use ::std::io::{Read, Seek};
use super::super::elf_ehdr::Elf_Ehdr_T;

elf_phdr!();

pub type Elf64_Phdr = Elf_Phdr<Elf64_Half, Elf64_Xword, Elf64_Addr, Elf64_Off>;
