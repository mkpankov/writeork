use ::std::fmt::{Display, Formatter};

#[repr(u16)]
#[derive(Debug,PartialEq,PartialOrd,Eq,Ord,Clone,Copy)]
#[allow(dead_code)]
pub enum ElfEhdrType {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE,
    ET_LOPROC = 0xff00,
    ET_HIPROC = 0xffff,
}

impl Display for ElfEhdrType {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        use self::ElfEhdrType::*;
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
