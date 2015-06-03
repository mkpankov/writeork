#![allow(non_camel_case_types)]

#[derive(Debug)]
struct Elf64_Half (u16);

const EI_NIDENT : usize = 16;

#[derive(Debug)]
struct Elf64_Ehdr<'a> {
    e_ident: &'a [u8; EI_NIDENT],
    e_type: Elf64_Half,
}

fn main() {
    ;
}
