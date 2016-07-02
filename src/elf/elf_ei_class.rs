use ::std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum ElfEiClass {
    ELFCLASSNONE,
    ELFCLASS32,
    ELFCLASS64,
}

impl Display for ElfEiClass {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        use self::ElfEiClass::*;
        let s = match *self {
            ELFCLASSNONE => "None",
            ELFCLASS32 => "ELF32",
            ELFCLASS64 => "ELF64",
        };
        write!(fmt, "{}", s)
    }
}
