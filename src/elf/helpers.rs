use super::b64::elf_ehdr::Elf_Ehdr as Elf64_Ehdr;
use std::io::{Read, Seek};

fn convert_byte_vec_to_ehdr_vec(
    v: Vec<u8>) 
    -> Result<Vec<Elf64_Ehdr>, ()> 
{
    let ehdr_size = ::std::mem::size_of::<Elf64_Ehdr>();

    assert_eq!(ehdr_size as usize, v.len());
    let mut r: Vec<Elf64_Ehdr> = unsafe {
        ::std::mem::transmute(v)
    };
    unsafe {
        r.set_len(1);
    }

    let proper_magic = &[0x7f, b'E', b'L', b'F'];
    let magic_ptr: *const [u8; 4] = unsafe {
        ::std::mem::transmute(r.as_ptr())
    };
    let magic = unsafe { &*magic_ptr };
    if proper_magic != magic {
        return Err(())
    }

    Ok(r)
}

fn convert_ehdr_vec_to_ehdr_box(
    mut v: Vec<Elf64_Ehdr>) 
    -> Box<Elf64_Ehdr>
{
    let ehdr = v.pop().unwrap();
    let r = Box::new(ehdr);
    r
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

    convert_ehdr_vec_to_ehdr_box(
        convert_byte_vec_to_ehdr_vec(
            b).unwrap())
}
