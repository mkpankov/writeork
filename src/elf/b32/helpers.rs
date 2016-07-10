use super::elf_ehdr::Elf_Ehdr as Elf32_Ehdr;
use super::elf_phdr::Elf_Phdr as Elf32_Phdr;
use super::primitive::*;
use ::std::io::{Read, Seek};

read_ehdr!(Elf32_Ehdr<Elf32_Half, Elf32_Word, Elf32_Addr, Elf32_Off>);
read_phdrs!(Elf32_Phdr, Elf32_Ehdr<Elf32_Half, Elf32_Word, Elf32_Addr, Elf32_Off>);
