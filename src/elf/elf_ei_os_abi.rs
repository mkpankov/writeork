use ::std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum ElfEiOsAbi {
    ELFOSABI_NONE = 0,
    ELFOSABI_HPUX = 1,
    ELFOSABI_NETBSD = 2,
    ELFOSABI_GNU = 3,
    ELFOSABI_SOLARIS = 6,
    ELFOSABI_AIX = 7,
    ELFOSABI_IRIX = 8,
    ELFOSABI_FREEBSD = 9,
    ELFOSABI_TRU64 = 10,
    ELFOSABI_MODESTO = 11,
    ELFOSABI_OPENBSD = 12,
    ELFOSABI_ARM_AEABI = 64,
    ELFOSABI_ARM = 97,
    ELFOSABI_STANDALONE = 255
}

#[allow(dead_code)]
const ELFOSABI_SYSV: u8 = ElfEiOsAbi::ELFOSABI_NONE as u8;
#[allow(dead_code)]
const ELFOSABI_LINUX: u8 = ElfEiOsAbi::ELFOSABI_GNU as u8;

impl Display for ElfEiOsAbi {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        use self::ElfEiOsAbi::*;
        let s = match *self {
            ELFOSABI_NONE => "UNIX - System V",
            ELFOSABI_HPUX => "HP-UX",
            ELFOSABI_NETBSD => "NetBSD",
            ELFOSABI_GNU => "GNU ELF",
            ELFOSABI_SOLARIS => "Sun Solaris",
            ELFOSABI_AIX => "IBM AIX",
            ELFOSABI_IRIX => "SGI Irix",
            ELFOSABI_FREEBSD => "FreeBSD",
            ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
            ELFOSABI_MODESTO => "Novell Modesto",
            ELFOSABI_OPENBSD => "OpenBSD",
            ELFOSABI_ARM_AEABI => "ARM EABI",
            ELFOSABI_ARM => "ARM",
            ELFOSABI_STANDALONE => "Standalone (embedded) application",
        };
        write!(fmt, "{}", s)
    }
}
