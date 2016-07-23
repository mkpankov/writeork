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

impl ElfEiClass {
    pub fn from_u8(byte: u8) -> Result<Self, ()> {
        // This furry code is due to being unable to case enum variants to u8
        // right in the match arm - so we have to introduce a guard, as guard 
        // can use arbitrary expressions 
        const NONE: ElfEiClass = ElfEiClass::ELFCLASSNONE;
        const CLASS32: ElfEiClass = ElfEiClass::ELFCLASS32;
        const CLASS64: ElfEiClass = ElfEiClass::ELFCLASS64;

        let result = match byte {
            b if b == NONE as u8 => NONE,
            b if b == CLASS32 as u8 => CLASS32,
            b if b == CLASS64 as u8 => CLASS64,
            _ => return Err(())
        };
        Ok(result)
    }
}
