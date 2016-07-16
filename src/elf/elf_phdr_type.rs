use ::std::fmt::{Display, Formatter};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ElfPhdrType {
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

impl Display for ElfPhdrType {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        use self::ElfPhdrType::*;
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

swap_copy_wrapper!(ElfPhdrType, u32);
to_host_copy_wrapper!(ElfPhdrType, u32);
