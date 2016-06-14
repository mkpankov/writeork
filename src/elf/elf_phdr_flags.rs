use ::std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Debug)]
pub struct ElfPhdrFlags {
    flags: u32,
}

impl Display for ElfPhdrFlags {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
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
