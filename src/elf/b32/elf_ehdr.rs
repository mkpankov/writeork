use ::std::io::{Read, Seek};

use super::primitive::*;

elf_ehdr!();

pub type Elf32_Ehdr = Elf_Ehdr<Elf32_Half, Elf32_Word, Elf32_Addr, Elf32_Off>;

read_ehdr!();
