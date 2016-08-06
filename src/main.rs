#![allow(non_camel_case_types)]

extern crate clap;
extern crate num;

// FIXME: Macro export from to_host is ugly. I'd like to hide to_host module.
#[macro_use]
mod to_host;

mod elf;

use elf::prelude::*;

use to_host::ToHostCopyStruct;

use clap::App;

use std::fs::File;

fn work(options: clap::ArgMatches) {
    let path = options.value_of("FILE").unwrap();

    let mut f = File::open(path).unwrap();

    let elf_class = read_class(&mut f).unwrap();

    if options.is_present("file-header") {
        let ehdr_outer: Box<Elf_Ehdr_TD>;

        match elf_class {
            ElfEiClass::ELFCLASS32 => {
                let ehdr:
                    Box<Elf32_Ehdr> =
                    Elf32_Ehdr::read_ehdr(&mut f);
                let ehdr = Box::new(ehdr.to_host_copy(&ehdr.get_endianness()));
                ehdr_outer = ehdr as Box<Elf_Ehdr_TD>;
            }
            ElfEiClass::ELFCLASS64 => {
                let ehdr:
                    Box<Elf64_Ehdr> =
                    Elf64_Ehdr::read_ehdr(&mut f);
                let ehdr = Box::new(ehdr.to_host_copy(&ehdr.get_endianness()));
                ehdr_outer = ehdr as Box<Elf_Ehdr_TD>;
            }
            ElfEiClass::ELFCLASSNONE => 
                panic!("This ELF file has ELFCLASSNONE. We can't get its bitness")
        }

        print!("{}", ehdr_outer);
    }

    if options.is_present("program-headers")
    || options.is_present("segments") {
        match elf_class {
            ElfEiClass::ELFCLASS32 => {
                let ehdr:
                    Box<Elf32_Ehdr> =
                    Elf32_Ehdr::read_ehdr(&mut f);        
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

                let phdrs = Elf32_Phdr::read_phdrs(&ehdr, &mut f);

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
            ElfEiClass::ELFCLASS64 => {
                let ehdr:
                    Box<Elf64_Ehdr> =
                    Elf64_Ehdr::read_ehdr(&mut f);        
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

                let phdrs = Elf64_Phdr::read_phdrs(&ehdr, &mut f);

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
            ElfEiClass::ELFCLASSNONE => {
                println!("This ELF file has ELFCLASSNONE. We can't get its bitness");
            }
        }
    }
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
