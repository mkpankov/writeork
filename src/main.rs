#![allow(non_camel_case_types)]

extern crate clap;

mod elf;

#[macro_use]
mod to_host;

use elf::prelude::*;

use to_host::{Endianness, ToHostInPlaceStruct, ToHostCopyStruct};
use to_host::swap_in_place::SwapInPlace;
use to_host::swap_copy::SwapCopy;
use to_host::to_host_in_place::ToHostInPlace;
use to_host::to_host_copy::ToHostCopy;

use clap::App;

use std::io::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs::File;

#[repr(u32)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfPhdrType {
    PT_NULL = 0,
    PT_LOAD = 1,
    PT_DYNAMIC = 2,
    PT_INTERP = 3,
    PT_NOTE = 4,
    PT_SHLIB = 5,
    PT_PHDR = 6,
    PT_TLS = 7,
    PT_NUM = 8,
    PT_LOOS = 0x60000000,
    PT_GNU_EH_FRAME = 0x6474e550,
    PT_GNU_STACK = 0x6474e551,
    PT_GNU_RELRO = 0x6474e552,
    PT_LOSUNW = 0x6ffffffa,
    PT_SUNWSTACK = 0x6ffffffb,
    PT_HISUNW = 0x6fffffff,
    PT_LOPROC = 0x70000000,
}

#[derive(Debug)]
#[repr(C)]
struct Elf64_Phdr {
    p_type: Elf64_Word,
    p_flags: Elf64_Word,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: Elf64_Xword,
    p_memsz: Elf64_Xword,
    p_align: Elf64_Xword,
}

impl Display for ElfPhdrType {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfPhdrType::*;
        let s = match *self {
            PT_NULL => "NULL",
            PT_LOAD => "LOAD",
            PT_DYNAMIC => "DYNAMIC",
            PT_INTERP => "INTERP",
            PT_NOTE => "NOTE",
            PT_SHLIB => "SHLIB",
            PT_PHDR => "PHDR",
            PT_TLS => "TLS",
            PT_NUM => "NUM",
            PT_LOOS => "LOOS",
            PT_GNU_EH_FRAME => "EH_FRAME",
            PT_GNU_STACK => "GNU_STACK",
            PT_GNU_RELRO => "GNU_RELRO",
            PT_LOSUNW => "LOSUNW",
            PT_SUNWSTACK => "SUNWBSS",
            PT_HISUNW => "HISUNW",
            PT_LOPROC => "LOPROC",
        };
        fmt.pad(s)
    }
}

#[repr(C)]
#[derive(Debug)]
struct ElfPhdrFlags {
    flags: u32,
}

impl Display for ElfPhdrFlags {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let maybe_r;
        let maybe_w;
        let maybe_x;

        if (self.flags & 0b100) != 0 {
            maybe_r = "R"
        } else {
            maybe_r = " "
        }
        if (self.flags & 0b010) != 0 {
            maybe_w = "W"
        } else {
            maybe_w = " "
        }
        if (self.flags & 0b001) != 0 {
            maybe_x = "E"
        } else {
            maybe_x = " "
        }

        write!(fmt, "{}{}{}", maybe_r, maybe_w, maybe_x)
    }
}

impl Elf64_Phdr {
    fn print_with_endianness(&self, e: &Endianness) {
        let p_type: ElfPhdrType = unsafe {
            std::mem::transmute(self.p_type.to_host_copy(e))
        };
        let p_flags: ElfPhdrFlags = unsafe {
            std::mem::transmute(self.p_flags.to_host_copy(e))
        };

        print!(
            concat!(
                "{: <15}",
                "{:#08x} ",
                "{:#018x} ",
                "{:#018x} ",
                "{:#08x} ",
                "{:#08x} ",
                "{:<3} ",
                "{:#x}",
                ),
            p_type,
            self.p_offset.to_host_copy(e),
            self.p_vaddr.to_host_copy(e),
            self.p_paddr.to_host_copy(e),
            self.p_filesz.to_host_copy(e),
            self.p_memsz.to_host_copy(e),
            p_flags,
            self.p_align.to_host_copy(e),
        );
    }
}

impl Elf64_Phdr {
    fn from_slice(buffer: &[u8]) -> &Elf64_Phdr {
        let phdr_ptr: *const Elf64_Phdr = unsafe {
            std::mem::transmute(buffer.as_ptr())
        };
        let phdr: &Elf64_Phdr = unsafe { &*phdr_ptr };

        phdr
    }
}

fn convert_byte_vec_to_ehdr_vec(
    v: Vec<u8>) -> Result<Vec<Elf64_Ehdr>, ()> {
    let ehdr_size = std::mem::size_of::<Elf64_Ehdr>();

    assert_eq!(ehdr_size as usize, v.len());
    let mut r: Vec<Elf64_Ehdr> = unsafe {
        std::mem::transmute(v)
    };
    unsafe {
        r.set_len(1);
    }

    let proper_magic = &[0x7f, b'E', b'L', b'F'];
    let magic_ptr: *const [u8; 4] = unsafe {
        std::mem::transmute(r.as_ptr())
    };
    let magic = unsafe { &*magic_ptr };
    if proper_magic != magic {
        return Err(())
    }

    Ok(r)
}

fn convert_ehdr_vec_to_ehdr_box(
    mut v: Vec<Elf64_Ehdr>) -> Box<Elf64_Ehdr>
{
    let ehdr = v.pop().unwrap();
    let r = Box::new(ehdr);
    r
}

fn read_ehdr<R: Read + Seek>(
    reader: &mut R)
    -> Box<Elf64_Ehdr>
{
    use std::io::SeekFrom;

    let ehdr_size = std::mem::size_of::<Elf64_Ehdr>();
    let ehdr_offset = 0;

    let mut b = Vec::<u8>::with_capacity(ehdr_size as usize);
    reader.seek(SeekFrom::Start(ehdr_offset)).unwrap();
    reader.take(ehdr_size as u64).read_to_end(&mut b).unwrap();

    convert_ehdr_vec_to_ehdr_box(
        convert_byte_vec_to_ehdr_vec(
            b).unwrap())
}

fn convert_byte_vec_to_phdrs_vec(
    v: Vec<u8>, phdr_num: u16, phdr_size: u16) -> Vec<Elf64_Phdr> {
    assert_eq!(phdr_num as usize * phdr_size as usize, v.len());
    let mut r: Vec<Elf64_Phdr> = unsafe {
        std::mem::transmute(v)
    };
    unsafe {
        r.set_len(phdr_num as usize);
    }
    r
}

fn read_phdrs<R: Read + Seek>(
    ehdr: &Elf64_Ehdr, reader: &mut R)
    -> Vec<Elf64_Phdr>
{
    use std::io::SeekFrom;

    let phdr_size = ehdr.get_phentsize() * ehdr.get_phnum();
    let phdr_offset = ehdr.get_phoff();
    let phdr_num = ehdr.get_phnum();

    let mut b = Vec::<u8>::with_capacity(phdr_size as usize * phdr_num as usize);
    reader.seek(SeekFrom::Start(phdr_offset)).unwrap();
    reader.take(phdr_size as u64 * phdr_num as u64).read_to_end(&mut b).unwrap();

    convert_byte_vec_to_phdrs_vec(b, phdr_num, phdr_size)
}

fn work(options: clap::ArgMatches) {
    let path = options.value_of("FILE").unwrap();

    let mut f = File::open(path).unwrap();

    let ehdr = read_ehdr(&mut f);

    if options.is_present("file-header") {
        print!("{}", ehdr);
    }

    if options.is_present("program-headers")
    || options.is_present("segments") {
        use to_host::ToHostCopyStruct;

        let ehdr = ehdr.to_host_copy(&ehdr.get_endianness());
        let e = ehdr.get_endianness();
        let e_type: ElfEhdrType = unsafe {
            std::mem::transmute(ehdr.get_type())
        };

        println!("");
        println!("Elf file type is {}", e_type);
        println!("Entry point {:#x}", ehdr.get_entry());
        println!(
            "There are {} program headers, starting at offset {}",
            ehdr.get_phnum(), ehdr.get_phoff());
        println!("");

        let phdrs = read_phdrs(&ehdr, &mut f);

        println!("Program headers:");
        println!(
            concat!(
                "  ",
                "Type           ",
                "Offset   ",
                "VirtAddr           ",
                "PhysAddr           ",
                "FileSiz  ",
                "MemSiz   ",
                "Flg ",
                "Align"));
        for phdr in phdrs {
            print!("  ");
            phdr.print_with_endianness(&e);
            println!("");
        }
    }
}

fn _static_asserts() {
    let ei_bytes: ElfIdent = unsafe {
        std::mem::uninitialized()
    };
    let _ei_named: ElfIdentNamed = unsafe {
        std::mem::transmute(ei_bytes)
    };

    let ehdr_type_bytes: Elf64_Half = unsafe {
        std::mem::uninitialized()
    };
    let _ehdr_type: ElfEhdrType = unsafe {
        std::mem::transmute(ehdr_type_bytes)
    };

    let ehdr_machine_bytes: Elf64_Half = unsafe {
        std::mem::uninitialized()
    };
    let _ehdr_machine: ElfEhdrMachine = unsafe {
        std::mem::transmute(ehdr_machine_bytes)
    };
}

fn process_args_and_work() {
    let options =
        App::new("writeork")
        .version("0.0.1")
        .author("Michael K. Pankov <work@michaelpankov.com>")
        .about(
            concat!("Parse and output information from ELF files.",
                    " Similar to readelf, but is not fully compatible."))
        .args_from_usage(
            "-h --file-header     'Display ELF file header'
             -l --program-headers 'Display the program headers'
                --segments        'An alias for --program-headers'
             <FILE> 'ELF file to parse'")
        .get_matches();
    work(options);
}

fn main() {
    process_args_and_work();
}

swap_in_place_wrapper!(ElfEhdrType, u16);
swap_in_place_wrapper!(ElfEhdrMachine, u16);
to_host_in_place_wrapper!(ElfEhdrType, u16);
to_host_in_place_wrapper!(ElfEhdrMachine, u16);

swap_copy_wrapper!(ElfEhdrType, u16);
swap_copy_wrapper!(ElfEhdrMachine, u16);
to_host_copy_wrapper!(ElfEhdrType, u16);
to_host_copy_wrapper!(ElfEhdrMachine, u16);

impl ToHostCopyStruct for Elf64_Phdr {
    fn to_host_copy(&self, endianness: &Endianness) -> Self {
        let e = endianness;
        Elf64_Phdr {
            p_type: self.p_type.to_host_copy(e),
            p_flags: self.p_flags.to_host_copy(e),
            p_offset: self.p_offset.to_host_copy(e),
            p_vaddr: self.p_vaddr.to_host_copy(e),
            p_paddr: self.p_paddr.to_host_copy(e),
            p_filesz: self.p_filesz.to_host_copy(e),
            p_memsz: self.p_memsz.to_host_copy(e),
            p_align: self.p_align.to_host_copy(e),
        }
    }
}
