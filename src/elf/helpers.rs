use super::b64::elf_ehdr::Elf_Ehdr as Elf64_Ehdr;
use std::io::{Read, Seek};

fn convert_byte_vec_to_ehdr_box(
    mut v: Vec<u8>)
    -> Result<Box<Elf64_Ehdr>, ()>
{
    let ehdr_size = ::std::mem::size_of::<Elf64_Ehdr>();

    assert_eq!(ehdr_size as usize, v.len());
    let bytes_ptr: *mut u8 = v.as_mut_ptr();
    ::std::mem::forget(v);

    let proper_magic = &[0x7f, b'E', b'L', b'F'];
    let magic_ptr: *const [u8; 4] = unsafe {
        ::std::mem::transmute(bytes_ptr)
    };
    let magic = unsafe { &*magic_ptr };
    if proper_magic != magic {
        return Err(())
    }

    let ehdr_ptr: *mut Elf64_Ehdr = unsafe {
        ::std::mem::transmute(bytes_ptr)
    };
    let ehdr_box: Box<Elf64_Ehdr> = unsafe {
        Box::from_raw(ehdr_ptr)
    };
    Ok(ehdr_box)
}

pub fn read_ehdr<R: Read + Seek>(
    reader: &mut R)
    -> Box<Elf64_Ehdr>
{
    use std::io::SeekFrom;

    let ehdr_size = ::std::mem::size_of::<Elf64_Ehdr>();
    let ehdr_offset = 0;

    let mut b = Vec::<u8>::with_capacity(ehdr_size as usize);
    reader.seek(SeekFrom::Start(ehdr_offset)).unwrap();
    reader.take(ehdr_size as u64).read_to_end(&mut b).unwrap();

    convert_byte_vec_to_ehdr_box(b).unwrap()
}
