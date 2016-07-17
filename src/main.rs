#![allow(non_camel_case_types)]

extern crate clap;
extern crate num;

// FIXME: Macro export from to_host is ugly. I'd like to hide to_host module.
#[macro_use]
mod to_host;

mod elf;
use elf::prelude::*;

use clap::App;

use std::fs::File;

fn work(options: clap::ArgMatches) {
    let path = options.value_of("FILE").unwrap();

    let mut f = File::open(path).unwrap();

    // FIXME: This is lazy guessing of bitness.
    // We first read Ehdr as 32-bit variant and then check 
    // if it's actually 32-bit. It works because ELF_EICLASS byte offset 
    // is always the same.
    let ehdr = Elf32_Ehdr::read_ehdr(&mut f);
    let elf_ident = ehdr.get_ident();
    let elf_class = elf_ident.get_class();

    if options.is_present("file-header") {
        if elf_class != ElfEiClass::ELFCLASS32 {
            // Reread the header as Elf64_Ehdr
            let ehdr:
                Box<Elf64_Ehdr> = 
                Elf64_Ehdr::read_ehdr(&mut f);
            print!("{}", ehdr);
        } else {
            print!("{}", ehdr);
        }
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

        match elf_class {
            // FIXME: This fugly code is due to ehdr and phdrs being of 
            // different type in different branches of control flow
            ElfEiClass::ELFCLASS32 => {
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
                let phdrs = Elf64_Phdr::read_phdrs(&*ehdr, &mut f);

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
