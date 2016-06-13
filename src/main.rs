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

#[repr(C)]
#[derive(Debug)]
struct ElfIdentNamed {
    ei_magic: [u8; 4],
    ei_class: ElfEiClass,
    ei_data: ElfEiData,
    ei_version: ElfEiVersion,
    ei_osabi: ElfEiOsAbi,
    ei_osabiversion: ElfEiAbiVersion,
    padding2: [u8; 7],
}


#[repr(u16)]
#[derive(Debug,PartialEq,PartialOrd,Eq,Ord,Clone,Copy)]
#[allow(dead_code)]
enum ElfEhdrType {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE,
    ET_LOPROC = 0xff00,
    ET_HIPROC = 0xffff,
}

impl Display for ElfEhdrType {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEhdrType::*;
        let s = match *self {
            ET_NONE => "NONE (No file type)",
            ET_REL => "REL (Relocatable file)",
            ET_EXEC => "EXEC (Executable file)",
            ET_DYN => "DYN (Shared object file)",
            ET_CORE => "CORE (Core file)",
            ref x if *x >= ET_LOPROC && *x <= ET_HIPROC => "Processor-specific",
            _ => "Unknown file type",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(u16)]
#[derive(Debug,Clone,Copy)]
#[allow(dead_code)]
enum ElfEhdrMachine {
    EM_NONE,
    EM_M32,
    EM_SPARC,
    EM_386,
    EM_68K,
    EM_88K,
    EM_860,
    EM_MIPS,
    EM_S370,
    EM_MIPS_RS3_LE,

    EM_PARISC = 15,

    EM_VPP500 = 17,
    EM_SPARC32PLUS,
    EM_960,
    EM_PPC,
    EM_PPC64,
    EM_S390,

    EM_V800 = 36,
    EM_FR20,
    EM_RH32,
    EM_RCE,
    EM_ARM,
    EM_FAKE_ALPHA,
    EM_SH,
    EM_SPARCV9,
    EM_TRICORE,
    EM_ARC,
    EM_H8_300,
    EM_H8_300H,
    EM_H8S,
    EM_H8_500,
    EM_IA_64,
    EM_MIPS_X,
    EM_COLDFIRE,
    EM_68HC12,
    EM_MMA,
    EM_PCP,
    EM_NCPU,
    EM_NDR1,
    EM_STARCORE,
    EM_ME16,
    EM_ST100,
    EM_TINYJ,
    EM_X86_64,
    EM_PDSP,

    EM_FX66 = 66,
    EM_ST9PLUS,
    EM_ST7,
    EM_68HC16,
    EM_68HC11,
    EM_68HC08,
    EM_68HC05,
    EM_SVX,
    EM_ST19,
    EM_VAX,
    EM_CRIS,
    EM_JAVELIN,
    EM_FIREPATH,
    EM_ZSP,
    EM_MMIX,
    EM_HUANY,
    EM_PRISM,
    EM_AVR,
    EM_FR30,
    EM_D10V,
    EM_D30V,
    EM_V850,
    EM_M32R,
    EM_MN10300,
    EM_MN10200,
    EM_PJ,
    EM_OPENRISC,
    EM_ARC_A5,
    EM_XTENSA,
    EM_AARCH64,
    EM_TILEPRO,
    EM_MICROBLAZE,
    EM_TILEGX,
    EM_NUM,

    EM_ALPHA = 0x9026,
}

impl Display for ElfEhdrMachine {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEhdrMachine::*;
        let s = match *self {
            EM_NONE => "No machine",
            EM_M32 => "AT&T WE 32100",
            EM_SPARC => "SUN SPARC",
            EM_386 => "Intel 80386",
            EM_68K => "Motorola m68k family",
            EM_88K => "Motorola m88k family",
            EM_860 => "Intel 80860",
            EM_MIPS => "MIPS R3000 big-endian",
            EM_S370 => "IBM System/370",
            EM_MIPS_RS3_LE => "MIPS R3000 little-endian",

            EM_PARISC => "HPPA",
            EM_VPP500 => "Fujitsu VPP500",
            EM_SPARC32PLUS => "Sun's \"v8plus\"",
            EM_960 => "Intel 80960",
            EM_PPC => "PowerPC",
            EM_PPC64 => "PowerPC 64-bit",
            EM_S390 => "IBM S390",

            EM_V800 => "NEC V800 series",
            EM_FR20 => "Fujitsu FR20",
            EM_RH32 => "TRW RH-32",
            EM_RCE => "Motorola RCE",
            EM_ARM => "ARM",
            EM_FAKE_ALPHA => "Digital Alpha",
            EM_SH => "Hitachi SH",
            EM_SPARCV9 => "SPARC v9 64-bit",
            EM_TRICORE => "Siemens Tricore",
            EM_ARC => "Argonaut RISC Core",
            EM_H8_300 => "Hitachi H8/300",
            EM_H8_300H => "Hitachi H8/300H",
            EM_H8S => "Hitachi H8S",
            EM_H8_500 => "Hitachi H8/500",
            EM_IA_64 => "Intel Merced",
            EM_MIPS_X => "Stanford MIPS-X",
            EM_COLDFIRE => "Motorola Coldfire",
            EM_68HC12 => "Motorola M68HC12",
            EM_MMA => "Fujitsu MMA Multimedia Accelerato",
            EM_PCP => "Siemens PCP",
            EM_NCPU => "Sony nCPU embeeded RISC",
            EM_NDR1 => "Denso NDR1 microprocessor",
            EM_STARCORE => "Motorola Start*Core processor",
            EM_ME16 => "Toyota ME16 processor",
            EM_ST100 => "STMicroelectronic ST100 processor",
            EM_TINYJ => "Advanced Logic Corp. Tinyj emb.fa",
            EM_X86_64 => "Advanced Micro Devices x86-64",
            EM_PDSP => "Sony DSP Processor",

            EM_FX66 => "Siemens FX66 microcontroller",
            EM_ST9PLUS => "STMicroelectronics ST9+ 8/16 mc",
            EM_ST7 => "STmicroelectronics ST7 8 bit mc",
            EM_68HC16 => "Motorola MC68HC16 microcontroller",
            EM_68HC11 => "Motorola MC68HC11 microcontroller",
            EM_68HC08 => "Motorola MC68HC08 microcontroller",
            EM_68HC05 => "Motorola MC68HC05 microcontroller",
            EM_SVX => "Silicon Graphics SVx",
            EM_ST19 => "STMicroelectronics ST19 8 bit mc",
            EM_VAX => "Digital VAX",
            EM_CRIS => "Axis Communications 32-bit embedded processor",
            EM_JAVELIN => "Infineon Technologies 32-bit embedded processor",
            EM_FIREPATH => "Element 14 64-bit DSP Processor",
            EM_ZSP => "LSI Logic 16-bit DSP Processor",
            EM_MMIX => "Donald Knuth's educational 64-bit processor",
            EM_HUANY => "Harvard University machine-independent object files",
            EM_PRISM => "SiTera Prism",
            EM_AVR => "Atmel AVR 8-bit microcontroller",
            EM_FR30 => "Fujitsu FR30",
            EM_D10V => "Mitsubishi D10V",
            EM_D30V => "Mitsubishi D30V",
            EM_V850 => "NEC v850",
            EM_M32R => "Mitsubishi M32R",
            EM_MN10300 => "Matsushita MN10300",
            EM_MN10200 => "Matsushita MN10200",
            EM_PJ => "picoJava",
            EM_OPENRISC => "OpenRISC 32-bit embedded processor",
            EM_ARC_A5 => "ARC Cores Tangent-A5",
            EM_XTENSA => "Tensilica Xtensa Architecture",
            EM_AARCH64 => "ARM AARCH64",
            EM_TILEPRO => "Tilera TILEPro",
            EM_MICROBLAZE => "Xilinx MicroBlaze",
            EM_TILEGX => "Tilera TILE-Gx",
            EM_ALPHA => "Alpha",
            _ => "Unknown machine",
        };
        write!(fmt, "{}", s)
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Elf64_Ehdr {
    e_ident: ElfIdent,
    e_type: ElfEhdrType,
    e_machine: ElfEhdrMachine,
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

impl Elf64_Ehdr {
    fn from_slice(buffer: &[u8]) -> Result<&Elf64_Ehdr, ()> {
        let proper_magic = &[0x7f, b'E', b'L', b'F'];
        let magic_ptr: *const [u8; 4] = unsafe {
            std::mem::transmute(buffer.as_ptr())
        };
        let magic = unsafe { &*magic_ptr };
        if proper_magic != magic {
            return Err(())
        }

        let ehdr_ptr: *const Elf64_Ehdr = unsafe {
            std::mem::transmute(buffer.as_ptr())
        };
        let ehdr: &Elf64_Ehdr = unsafe { &*ehdr_ptr };

        Ok(ehdr)
    }
}

impl Display for Elf64_Ehdr {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let ehdr_ident: &ElfIdentNamed = unsafe {
            std::mem::transmute(&self.e_ident)
        };

        let e = self.get_endianness();

        write!(
            fmt,
            concat!(
                "ELF Header:\n",
                "  Magic:   {}\n",
                "  Class:                             {}\n",
                "  Data:                              {}\n",
                "  Version:                           {}\n",
                "  OS/ABI:                            {}\n",
                "  ABI Version:                       {}\n",
                "  Type:                              {}\n",
                "  Machine:                           {}\n",
                "  Version:                           {:#x}\n",
                "  Entry point address:               {:#x}\n",
                "  Start of program headers:          {} (bytes into file)\n",
                "  Start of section headers:          {} (bytes into file)\n",
                "  Flags:                             {:#x}\n",
                "  Size of this header:               {} (bytes)\n",
                "  Size of program headers:           {} (bytes)\n",
                "  Number of program headers:         {}\n",
                "  Size of section headers:           {} (bytes)\n",
                "  Number of section headers:         {}\n",
                "  Section header string table index: {}\n",
                ),
            self.e_ident,
            ehdr_ident.ei_class,
            ehdr_ident.ei_data,
            ehdr_ident.ei_version,
            ehdr_ident.ei_osabi,
            ehdr_ident.ei_osabiversion,
            self.e_type.to_host_copy(&e),
            self.e_machine.to_host_copy(&e),
            self.e_version.to_host_copy(&e),
            self.e_entry.to_host_copy(&e),
            self.e_phoff.to_host_copy(&e),
            self.e_shoff.to_host_copy(&e),
            self.e_flags.to_host_copy(&e),
            self.e_ehsize.to_host_copy(&e),
            self.e_phentsize.to_host_copy(&e),
            self.e_phnum.to_host_copy(&e),
            self.e_shentsize.to_host_copy(&e),
            self.e_shnum.to_host_copy(&e),
            self.e_shstrndx.to_host_copy(&e))
    }
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

    let phdr_size = ehdr.e_phentsize * ehdr.e_phnum;
    let phdr_offset = ehdr.e_phoff;
    let phdr_num = ehdr.e_phnum;

    let mut b = Vec::<u8>::with_capacity(phdr_size as usize * phdr_num as usize);
    reader.seek(SeekFrom::Start(phdr_offset)).unwrap();
    reader.take(phdr_size as u64 * phdr_num as u64).read_to_end(&mut b).unwrap();

    convert_byte_vec_to_phdrs_vec(b, phdr_num, phdr_size)
}

impl Elf64_Ehdr {
    fn get_endianness(&self) -> Endianness {
        let ehdr_ptr: *mut Elf64_Ehdr = unsafe {
            std::mem::transmute(self)
        };
        let ehdr: &mut Elf64_Ehdr = unsafe { &mut *ehdr_ptr };
        let ehdr_ident: &ElfIdentNamed = unsafe {
            std::mem::transmute(&ehdr.e_ident)
        };

        ehdr_ident.ei_data.get_endianness()
    }
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
            std::mem::transmute(ehdr.e_type)
        };

        println!("");
        println!("Elf file type is {}", e_type);
        println!("Entry point {:#x}", ehdr.e_entry);
        println!(
            "There are {} program headers, starting at offset {}",
            ehdr.e_phnum, ehdr.e_phoff);
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

impl ToHostInPlaceStruct for Elf64_Ehdr {
    fn to_host_in_place(&mut self, endianness: &Endianness) {
        let e = endianness;
        self.e_type.to_host_in_place(e);
        self.e_machine.to_host_in_place(e);
        self.e_version.to_host_in_place(e);
        self.e_entry.to_host_in_place(e);
        self.e_phoff.to_host_in_place(e);
        self.e_shoff.to_host_in_place(e);
        self.e_flags.to_host_in_place(e);
        self.e_ehsize.to_host_in_place(e);
        self.e_phentsize.to_host_in_place(e);
        self.e_phnum.to_host_in_place(e);
        self.e_shentsize.to_host_in_place(e);
        self.e_shnum.to_host_in_place(e);
        self.e_shstrndx.to_host_in_place(e);
    }
}

impl ToHostCopyStruct for Elf64_Ehdr {
    fn to_host_copy(&self, endianness: &Endianness) -> Self {
        let e = endianness;
        Elf64_Ehdr {
            e_ident: self.e_ident,
            e_type: self.e_type.to_host_copy(e),
            e_machine: self.e_machine.to_host_copy(e),
            e_version: self.e_version.to_host_copy(e),
            e_entry: self.e_entry.to_host_copy(e),
            e_phoff: self.e_phoff.to_host_copy(e),
            e_shoff: self.e_shoff.to_host_copy(e),
            e_flags: self.e_flags.to_host_copy(e),
            e_ehsize: self.e_ehsize.to_host_copy(e),
            e_phentsize: self.e_phentsize.to_host_copy(e),
            e_phnum: self.e_phnum.to_host_copy(e),
            e_shentsize: self.e_shentsize.to_host_copy(e),
            e_shnum: self.e_shnum.to_host_copy(e),
            e_shstrndx: self.e_shstrndx.to_host_copy(e),
        }
    }
}

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
