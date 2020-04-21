#[macro_use] extern crate nom;
#[macro_use] extern crate nom_trace;
use nom::character::streaming::digit0;

use nom::character::is_alphabetic;

fn main(){

    named!( o<&[u8], Option<&[u8]> >, opt!( tag!( "abcd" ) ) );

    let a = b"abcdef";
    let b = b"bcdefg";
    assert_eq!(o(&a[..]), Ok((&b"ef"[..], Some(&b"abcd"[..]))));
    assert_eq!(o(&b[..]), Ok((&b"bcdefg"[..], None)));

    

    println!("{:?}", o(&a[..]));

    named!(opt_tag<&[u8], Option<&[u8]> >, opt!( digit0 ));
    let s = b"123hgfgood";
    println!("{:?}", opt_tag(&s[..]));
    println!("{:?}", b"1");


    // ------------------ nom trace --------------------------
    named!(parser<&str, Vec<&str>>,
        //wrap a parser with tr!() to add a trace point
        tr!(preceded!(
        tr!(tag!("data: ")),
        tr!(delimited!(
            tag!("("),
            separated_list!(
            tr!(tag!(",")),
            tr!(digit0)
            ),
            tr!(tag!(")"))
        ))
        ))
    );

    println!("parsed: {:?}", parser("data: (1,2,3)"));
    
    // prints the last parser trace
    print_trace!();

    // the list of trace events can be cleared
    reset_trace!();

}