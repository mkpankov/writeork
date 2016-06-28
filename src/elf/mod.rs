pub mod elf_ident;
pub mod elf_ei_class;
pub mod elf_ei_data;
pub mod elf_ei_version;
pub mod elf_ei_os_abi;
pub mod elf_ei_abi_version;
pub mod elf_ident_named;
pub mod elf_ehdr_type;
pub mod elf_ehdr_machine;
pub mod elf_phdr_type;
pub mod elf_phdr_flags;

#[macro_use]
mod elf_ehdr;
#[macro_use]
mod elf_phdr;

mod b32;
mod b64;

pub use ::elf::b32::elf_ehdr::Elf_Ehdr as Elf32_Ehdr;
pub use ::elf::b32::elf_phdr::Elf_Phdr as Elf32_Phdr;

pub use ::elf::b64::elf_ehdr::Elf_Ehdr as Elf64_Ehdr;
pub use ::elf::b64::elf_phdr::Elf_Phdr as Elf64_Phdr;

pub mod prelude;
pub mod helpers;

pub use ::elf::helpers::read_ehdr;