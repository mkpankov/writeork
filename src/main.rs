#![feature(libc)]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::funcs::posix88::mman::{mmap, munmap};
use libc::funcs::posix88::fcntl::open as c_open;
use libc::funcs::posix88::unistd::close;
use libc::consts::os::posix88::O_RDONLY;
use libc::consts::os::posix88::PROT_READ;
use libc::consts::os::posix88::{MAP_FIXED, MAP_FILE, MAP_ANON, MAP_PRIVATE, MAP_FAILED};
use libc::types::os::arch::posix88::mode_t;
use libc::types::common::c95::{c_void};
use libc::types::os::arch::c95::{c_char, c_int};
use std::ffi::CString;
use std::mem;

#[derive(Debug)]
struct Elf64_Half (u16);

const EI_NIDENT : usize = 16;

#[derive(Debug)]
struct Elf64_Ehdr<'a> {
    e_ident: &'a [u8; EI_NIDENT],
    e_type: Elf64_Half,
}

fn open() -> i32 {
    let path = "target/relf";
    let s = CString::from_slice(path.as_bytes());
    let oflag : c_int = O_RDONLY;
    let mode : mode_t = 0 as mode_t;
    let r = unsafe {
        c_open(s.as_ptr(), oflag, mode)
    };
    r
}

fn map(fd: i32) {
    let a : *mut c_void = 0 as *mut c_void;
    let r = unsafe {
        mmap(a, mem::size_of::<Elf64_Ehdr>() as u64, PROT_READ, MAP_PRIVATE, fd, 0)
    };
    let res : usize = unsafe { mem::transmute(r) };
    println!("{:x}", res);
}

fn main() {
    let fd = open();
    map(fd);
}
