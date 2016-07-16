use super::elf_ehdr::Elf64_Ehdr;
use super::elf_phdr::Elf64_Phdr;
use ::std::io::{Read, Seek};

read_phdrs!(Elf64_Phdr, Elf64_Ehdr);
