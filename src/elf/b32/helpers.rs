use super::elf_ehdr::Elf32_Ehdr;
use super::elf_phdr::Elf32_Phdr;
use ::std::io::{Read, Seek};

read_phdrs!(Elf32_Phdr, Elf32_Ehdr);
