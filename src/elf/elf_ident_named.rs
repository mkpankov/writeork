use ::std::fmt::{Display, Formatter};

use super::prelude::{ElfEiClass, ElfEiData, ElfEiVersion, ElfEiOsAbi, ElfEiAbiVersion};

#[repr(C)]
#[derive(Debug)]
pub struct ElfIdentNamed {
    ei_magic: [u8; 4],
    ei_class: ElfEiClass,
    ei_data: ElfEiData,
    ei_version: ElfEiVersion,
    ei_osabi: ElfEiOsAbi,
    ei_osabiversion: ElfEiAbiVersion,
    _padding: [u8; 7],
}

impl Display for ElfIdentNamed {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        write!(
            fmt,
            concat!(
                "  Class:                             {}\n",
                "  Data:                              {}\n",
                "  Version:                           {}\n",
                "  OS/ABI:                            {}\n",
                "  ABI Version:                       {}\n",
                ),
            self.ei_class,
            self.ei_data,
            self.ei_version,
            self.ei_osabi,
            self.ei_osabiversion,
        )
    }
}

impl ElfIdentNamed {
    pub fn get_endianness(&self) -> ::to_host::Endianness {
        self.ei_data.get_endianness()
    }
}