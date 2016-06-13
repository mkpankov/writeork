use ::std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum ElfEiData {
    ELFDATANONE,
    ELFDATA2LSB,
    ELFDATA2MSB,
}

impl Display for ElfEiData {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        use self::ElfEiData::*;
        let s = match *self {
            ELFDATANONE => "None",
            ELFDATA2LSB => "2's complement, little endian",
            ELFDATA2MSB => "2's complement, big endian",
        };
        write!(fmt, "{}", s)
    }
}

impl ElfEiData {
    pub fn get_endianness(&self) -> ::to_host::Endianness {
        use self::ElfEiData::*;
        use ::to_host::Endianness::*;

        match *self {
            ELFDATA2MSB => BE,
            ELFDATA2LSB => LE,
            ELFDATANONE => panic!("Unknown data format"),
        } 
    }
}