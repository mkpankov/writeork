# Endianness

The problem is that we only can read single-byte fields of ELF header w/o
determining the endianness.

Determining it is easy, there's a specific field.

Then, we have some options:

* Read the headers as-is and only convert some, needed, fields when printing
them;

* Parse headers manually with `byteorder` crate;

* Parse headers with `nom`.

# Using endianness conversion from `std`

We don't use it as using `from_be()` implemented on `u16` means we need
`num::PrimInt` trait bound on our wrapper types (i.e. `ElfEhdrMachine` is `u16`,
but is not `PrimInt`).

The way it's currently implemented is via self-sufficient implementation that
can be added to any primitive type by implementation of `SwapCopy` and
`SwapInPlace` traits for copying endianness change and in-place endianness
change correspondingly.

For structs, there's `ToHostCopyStruct` and `ToHostInPlaceStruct`, that
implement copying and in-place conversion of all the fields of the struct
correspondingly.
