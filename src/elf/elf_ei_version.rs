use ::std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum ElfEiVersion {
    EV_NONE,
    EV_CURRENT,
}

impl Display for ElfEiVersion {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        use self::ElfEiVersion::*;
        let s = match *self {
            EV_NONE => "None",
            EV_CURRENT => "1 (current)",
        };
        write!(fmt, "{}", s)
    }
}
