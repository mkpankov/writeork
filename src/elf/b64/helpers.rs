use super::elf_ehdr::Elf_Ehdr as Elf64_Ehdr;
use super::elf_phdr::Elf_Phdr as Elf64_Phdr;
use ::std::io::{Read, Seek};

read_ehdr!(Elf64_Ehdr);
read_phdrs!(Elf64_Phdr, Elf64_Ehdr);
