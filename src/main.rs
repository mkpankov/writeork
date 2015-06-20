#![allow(non_camel_case_types)]

extern crate clap;

use clap::App;

use std::io::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs::File;

type Elf64_Half = u16;

type Elf64_Word = u32;

type Elf64_Addr = u64;

type Elf64_Off = u64;

type Elf64_Xword = u64;

const EI_NIDENT : usize = 16;

#[repr(C)]
#[derive(Debug)]
struct ElfIdent {
    data: [u8; EI_NIDENT],
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
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

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiData {
    ELFDATANONE,
    ELFDATA2LSB,
    ELFDATA2MSB,
}

impl Display for ElfEiData {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiData::*;
        let s = match *self {
            ELFDATANONE => "None",
            ELFDATA2LSB => "2's complement, little endian",
            ELFDATA2MSB => "2's complement, big endian",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiVersion {
    EV_NONE,
    EV_CURRENT,
}

impl Display for ElfEiVersion {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiVersion::*;
        let s = match *self {
            EV_NONE => "None",
            EV_CURRENT => "1 (current)",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiOsAbi {
    ELFOSABI_NONE = 0,
    ELFOSABI_HPUX = 1,
    ELFOSABI_NETBSD = 2,
    ELFOSABI_GNU = 3,
    ELFOSABI_SOLARIS = 6,
    ELFOSABI_AIX = 7,
    ELFOSABI_IRIX = 8,
    ELFOSABI_FREEBSD = 9,
    ELFOSABI_TRU64 = 10,
    ELFOSABI_MODESTO = 11,
    ELFOSABI_OPENBSD = 12,
    ELFOSABI_ARM_AEABI = 64,
    ELFOSABI_ARM = 97,
    ELFOSABI_STANDALONE = 255
}

#[allow(dead_code)]
const ELFOSABI_SYSV: u8 = ElfEiOsAbi::ELFOSABI_NONE as u8;
#[allow(dead_code)]
const ELFOSABI_LINUX: u8 = ElfEiOsAbi::ELFOSABI_GNU as u8;

impl Display for ElfEiOsAbi {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiOsAbi::*;
        let s = match *self {
            ELFOSABI_NONE => "UNIX - System V",
            ELFOSABI_HPUX => "HP-UX",
            ELFOSABI_NETBSD => "NetBSD",
            ELFOSABI_GNU => "GNU ELF",
            ELFOSABI_SOLARIS => "Sun Solaris",
            ELFOSABI_AIX => "IBM AIX",
            ELFOSABI_IRIX => "SGI Irix",
            ELFOSABI_FREEBSD => "FreeBSD",
            ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
            ELFOSABI_MODESTO => "Novell Modesto",
            ELFOSABI_OPENBSD => "OpenBSD",
            ELFOSABI_ARM_AEABI => "ARM EABI",
            ELFOSABI_ARM => "ARM",
            ELFOSABI_STANDALONE => "Standalone (embedded) application",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
struct ElfEiAbiVersion {
    data: u8,
}

impl Display for ElfEiAbiVersion {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.data)
    }
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

impl Display for ElfIdent {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for b in self.data.iter() {
            try!(
                write!(
                    fmt, "{:02x} ", b));
        }
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug,PartialEq,PartialOrd,Eq,Ord)]
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
#[derive(Debug)]
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
    EM_VPP500,
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
#[derive(Debug)]
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

trait BinaryDeserialize {
    fn to_host(&mut self, endianness: Endianness);
}

#[allow(dead_code)]
enum Endianness {
    LE,
    BE,
}

trait FromInPlace {
    fn from_be_in_place(&mut self);
    fn from_le_in_place(&mut self);
}

impl FromInPlace for u64 {
    fn from_be_in_place(&mut self) {
        let size = std::mem::size_of::<u64>();
        assert_eq!(size, 8);
        let self_ptr: *mut [u8; 8] = unsafe {
            std::mem::transmute(self)
        };

        for i in 0..size / 2 {
            unsafe {
                std::mem::swap(&mut (*self_ptr)[i],
                               &mut (*self_ptr)[size - i - 1])
            };
        }
    }
    fn from_le_in_place(&mut self) {
        unreachable!();
    }
}

impl BinaryDeserialize for Elf64_Addr {
    fn to_host(&mut self, endianness: Endianness) {
        use Endianness::*;
        match endianness {
            BE => u64::from_be_in_place(self),
            LE => u64::from_le_in_place(self),
        };
    }
}

impl BinaryDeserialize for Elf64_Ehdr {
    fn to_host(&mut self, endianness: Endianness) {
        self.e_entry.to_host(endianness);
    }
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
                "  Section header string table index: {}",
                ),
            self.e_ident,
            ehdr_ident.ei_class,
            ehdr_ident.ei_data,
            ehdr_ident.ei_version,
            ehdr_ident.ei_osabi,
            ehdr_ident.ei_osabiversion,
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

fn work(options: clap::ArgMatches) {
    let path = options.value_of("FILE").unwrap();
    let f = File::open(path).unwrap();
    let mut b = Vec::<u8>::with_capacity(std::mem::size_of::<Elf64_Ehdr>());
    (&f).take(std::mem::size_of::<Elf64_Ehdr>() as u64).read_to_end(&mut b).unwrap();

    let proper_magic = &[0x7f, b'E', b'L', b'F'];
    let magic_ptr: *const [u8; 4] = unsafe {
        std::mem::transmute(b.as_ptr())
    };
    let magic = unsafe { &*magic_ptr };
    if proper_magic != magic {
        panic!("Not an ELF file");
    }

    if options.is_present("file-header") {
        use ElfEiData::*;
        use Endianness::*;

        let ehdr_ptr: *mut Elf64_Ehdr = unsafe {
            std::mem::transmute(b.as_ptr())
        };
        let mut ehdr: &mut Elf64_Ehdr = unsafe { &mut *ehdr_ptr };
        let ehdr_ident: &ElfIdentNamed = unsafe {
            std::mem::transmute(&ehdr.e_ident)
        };

        let e = match ehdr_ident.ei_data {
            ELFDATA2MSB => BE,
            ELFDATA2LSB => LE,
            ELFDATANONE => panic!("Unknown data format"),
        };
        ehdr.to_host(e);

        println!("{}", ehdr);
    }

    if options.is_present("program-headers")
    || options.is_present("segments") {
        use std::io::SeekFrom;

        let ehdr_ptr: *const Elf64_Ehdr = unsafe {
            std::mem::transmute(b.as_ptr())
        };
        let ehdr: &Elf64_Ehdr = unsafe { &*ehdr_ptr };

        let phdr_size = ehdr.e_phentsize * ehdr.e_phnum;
        let phdr_offset = ehdr.e_phoff;

        let mut b2 = Vec::<u8>::with_capacity(phdr_size as usize);
        (&f).seek(SeekFrom::Start(phdr_offset)).unwrap();
        (&f).take(phdr_size as u64).read_to_end(&mut b2).unwrap();

        let phdr_ptr: *const Elf64_Phdr = unsafe {
            std::mem::transmute(b2.as_ptr())
        };
        let phdr: &Elf64_Phdr = unsafe { &*phdr_ptr };

        println!("{:?}", phdr);
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
