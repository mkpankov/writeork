use super::elf_ehdr::Elf_Ehdr as Elf64_Ehdr;
use ::std::io::{Read, Seek};

read_ehdr!(Elf64_Ehdr);