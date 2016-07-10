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
#[macro_use]
mod helpers;

mod b32;
mod b64;

pub use ::elf::b32::elf_ehdr::Elf32_Ehdr;
pub use ::elf::b32::elf_phdr::Elf_Phdr as Elf32_Phdr;

pub use ::elf::b64::elf_ehdr::Elf64_Ehdr;
pub use ::elf::b64::elf_phdr::Elf_Phdr as Elf64_Phdr;

pub mod prelude;

pub use ::elf::b32::helpers::read_phdrs as read_elf32_phdrs;
pub use ::elf::b64::helpers::read_phdrs as read_elf64_phdrs;

fn _static_asserts() {
    let ei_bytes: elf_ident::ElfIdent = unsafe {
        ::std::mem::uninitialized()
    };
    let _ei_named: elf_ident_named::ElfIdentNamed = unsafe {
        ::std::mem::transmute(ei_bytes)
    };

    let ehdr_type_bytes: b64::primitive::Elf64_Half = unsafe {
        ::std::mem::uninitialized()
    };
    let _ehdr_type: elf_ehdr_type::ElfEhdrType = unsafe {
        ::std::mem::transmute(ehdr_type_bytes)
    };

    let ehdr_machine_bytes: b64::primitive::Elf64_Half = unsafe {
        ::std::mem::uninitialized()
    };
    let _ehdr_machine: elf_ehdr_machine::ElfEhdrMachine = unsafe {
        ::std::mem::transmute(ehdr_machine_bytes)
    };
}
