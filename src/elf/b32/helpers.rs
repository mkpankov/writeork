use super::elf_ehdr::Elf_Ehdr as Elf32_Ehdr;
use super::elf_phdr::Elf_Phdr as Elf32_Phdr;
use super::super::elf_ident_named::EI_MAGIC_SIZE;
use ::std::io::{Read, Seek};

read_ehdr!(Elf32_Ehdr);
read_phdrs!(Elf32_Phdr, Elf32_Ehdr);
