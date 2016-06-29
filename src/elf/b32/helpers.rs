use super::elf_ehdr::Elf_Ehdr as Elf32_Ehdr;
use ::std::io::{Read, Seek};

read_ehdr!(Elf32_Ehdr);
