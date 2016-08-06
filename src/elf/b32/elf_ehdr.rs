use ::std::io::{Read, Seek};

use super::primitive::*;
use super::super::elf_ehdr::{Elf_Ehdr_T, Elf_Ehdr_TD};

elf_ehdr!();

pub type Elf32_Ehdr = Elf_Ehdr<Elf32_Half, Elf32_Word, Elf32_Addr, Elf32_Off>;
