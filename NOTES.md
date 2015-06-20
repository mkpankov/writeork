# Endianness

The problem is that we only can read single-byte fields of ELF header w/o
determining the endianness.

Determining it is easy, there's a specific field.

Then, we have some options:

* Read the headers as-is and only convert some, needed, fields when printing
them;

* Parse headers manually with `byteorder` crate;

* Parse headers with `nom`.
