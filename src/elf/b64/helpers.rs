use super::elf_ehdr::Elf_Ehdr as Elf64_Ehdr;
use super::elf_phdr::Elf_Phdr as Elf64_Phdr;
use super::primitive::*;
use ::std::io::{Read, Seek};

read_phdrs!(Elf64_Phdr, Elf64_Ehdr<Elf64_Half, Elf64_Word, Elf64_Addr, Elf64_Off>);
