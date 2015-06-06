#![allow(non_camel_case_types)]

use std::io::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs::File;

#[derive(Debug)]
struct Elf64_Half (u16);

#[derive(Debug)]
struct Elf64_Word (u32);

#[derive(Debug)]
struct Elf64_Addr (u64);

#[derive(Debug)]
struct Elf64_Off (u64);

const EI_NIDENT : usize = 16;
const EI_CLASS : usize = 4;

#[repr(C)]
#[derive(Debug)]
struct ElfIdent {
    data: [u8; EI_NIDENT],
}

#[repr(u8)]
#[derive(Debug)]
enum ElfEiClass {
    ELFCLASSNONE,
    ELFCLASS32,
    ELFCLASS64,
}

impl Display for ElfEiClass {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiClass::*;
        let s = match *self {
            ELFCLASSNONE => "None",
            ELFCLASS32 => "ELF32",
            ELFCLASS64 => "ELF64",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug)]
struct ElfIdentNamed {
    padding1: [u8; 3],
    ei_class: ElfEiClass,
    padding2: [u8; 12],
}

impl Display for ElfIdent {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for (i, b) in self.data.iter().enumerate() {
            if i < self.data.iter().count() - 1 {
                try!(
                    write!(
                        fmt, "{:02x} ", b));
            } else {
                try!(
                    write!(
                        fmt, "{:02x}", b));
            }
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
struct Elf64_Ehdr {
    e_ident: ElfIdent,
    e_type: Elf64_Half,
    e_machine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half
}

impl Display for Elf64_Ehdr {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let ehdr_ident: &ElfIdentNamed = unsafe {
            std::mem::transmute(&self.e_ident)
        };

        write!(
            fmt,
            concat!(
                "ELF Header:\n",
                "  Magic:   {}\n",
                "  Class:                             {}\n",
                "  Data:                              {:?}\n",
                "  Version:                           {:?}\n",
                "  OS ABI:                            {:?}\n",
                "  ABI Version:                       {:?}\n",
                "  Type:                              {:?}\n",
                "  Machine:                           {:?}\n",
                "  Version:                           {:?}\n",
                "  Entry point address:               {:?}\n",
                "  Start of program headers:          {:?}\n",
                "  Start of section headers:          {:?}\n",
                "  Flags:                             {:?}\n",
                "  Size of this header:               {:?}\n",
                "  Size of program headers:           {:?}\n",
                "  Number of program headers:         {:?}\n",
                "  Size of section headers:           {:?}\n",
                "  Number of section headers:         {:?}\n",
                "  Section header string table index: {:?}\n",
                ),
            self.e_ident,
            ehdr_ident.ei_class,
            "data",
            self.e_version,
            "OS ABI",
            "ABI VERSION",
            self.e_type,
            self.e_machine,
            self.e_version,
            self.e_entry,
            self.e_phoff,
            self.e_shoff,
            self.e_flags,
            self.e_ehsize,
            self.e_phentsize,
            self.e_phnum,
            self.e_shentsize,
            self.e_shnum,
            self.e_shstrndx)
    }
}

fn work() {
    let f = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut b = Vec::<u8>::with_capacity(std::mem::size_of::<Elf64_Ehdr>());
    f.take(std::mem::size_of::<Elf64_Ehdr>() as u64).read_to_end(&mut b).unwrap();
    let ehdr: *const Elf64_Ehdr = unsafe {
        std::mem::transmute(b.as_ptr())
    };
    let e: &Elf64_Ehdr = unsafe { &*ehdr };
    println!("{}", e);
}

fn _static_asserts() {
    let ei_bytes: ElfIdent = unsafe {
        std::mem::uninitialized()
    };
    let _ei_named: ElfIdentNamed =
        unsafe {
            std::mem::transmute(ei_bytes)
        };
}

fn main() {
    work();
}
