use ::std::fmt::{Display, Formatter};
use ::std::io::{Read, Seek};

use super::prelude::{ElfEiClass, ElfEiData, ElfEiVersion, ElfEiOsAbi, ElfEiAbiVersion};

pub const EI_MAGIC_SIZE: usize = 4;
const EI_MAGIC_CLASS_SIZE: usize = EI_MAGIC_SIZE + 1;

type ElfEiMagic = [u8; EI_MAGIC_SIZE]; 

#[repr(C)]
#[derive(Debug)]
pub struct ElfIdentNamed {
    ei_magic: ElfEiMagic,
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
    pub fn get_class(&self) -> ElfEiClass {
        self.ei_class
    }
    #[allow(dead_code)]
    pub fn read_class<R: Read + Seek>(mut reader: R) -> ElfEiClass {
        use std::io::SeekFrom;

        let offset = 0;

        let mut b = [0; EI_MAGIC_CLASS_SIZE];
        reader.seek(SeekFrom::Start(offset)).unwrap();
        reader.read_exact(&mut b).unwrap();

        let class: ElfEiClass = unsafe {
            ::std::mem::transmute(b[EI_MAGIC_SIZE])
        };
        class
    }
}

pub fn asserts() {
    let elf_ident_magic_class = 
        ::std::mem::size_of::<ElfEiMagic>()
      + ::std::mem::size_of::<ElfEiClass>();
    assert_eq!(elf_ident_magic_class, EI_MAGIC_CLASS_SIZE);
}