use ::std::io::{Read, Seek};

use super::primitive::*;
use super::super::elf_ehdr::Elf_Ehdr_T;

elf_ehdr!();

pub type Elf64_Ehdr = Elf_Ehdr<Elf64_Half, Elf64_Word, Elf64_Addr, Elf64_Off>;
