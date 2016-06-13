use ::std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct ElfEiAbiVersion {
    data: u8,
}

impl Display for ElfEiAbiVersion {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        write!(fmt, "{}", self.data)
    }
}
