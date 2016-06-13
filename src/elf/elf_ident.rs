use ::std::fmt::{Display, Formatter};

const EI_NIDENT : usize = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ElfIdent {
    data: [u8; EI_NIDENT],
}

impl Display for ElfIdent {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        for b in self.data.iter() {
            try!(
                write!(
                    fmt, "{:02x} ", b));
        }
        Ok(())
    }
}
