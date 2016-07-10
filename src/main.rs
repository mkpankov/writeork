#![allow(non_camel_case_types)]

extern crate clap;

#[macro_use]
mod to_host;

mod elf;
use elf::prelude::*;

use clap::App;

use std::io::prelude::*;
use std::fs::File;

fn convert_byte_vec_to_ehdr_vec(
    v: Vec<u8>) -> Result<Vec<Elf64_Ehdr<Elf64_Word>>, ()> {
    let ehdr_size = std::mem::size_of::<Elf64_Ehdr<Elf64_Word>>();

    assert_eq!(ehdr_size as usize, v.len());
    let mut r: Vec<Elf64_Ehdr<Elf64_Word>> = unsafe {
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
    mut v: Vec<Elf64_Ehdr<Elf64_Word>>) -> Box<Elf64_Ehdr<Elf64_Word>>
{
    let ehdr = v.pop().unwrap();
    let r = Box::new(ehdr);
    r
}

fn read_ehdr<R: Read + Seek>(
    reader: &mut R)
    -> Box<Elf64_Ehdr<Elf64_Word>>
{
    use std::io::SeekFrom;

    let ehdr_size = std::mem::size_of::<Elf64_Ehdr<Elf64_Word>>();
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
    ehdr: &Elf64_Ehdr<Elf64_Word>, reader: &mut R)
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
