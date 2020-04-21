#[macro_use]
extern crate nom;

named!(opt_tag<Option<&[u8]>>, opt!(tag!("a")));
