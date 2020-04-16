extern crate proc_macro;
extern crate regex;

use proc_macro::TokenStream;
use std::str::FromStr;
use regex::Regex;

#[proc_macro_derive(HelloWorld)]
//此函数名是可以是任意的
pub fn hello_world(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    println!("{}", s);
    //获取derive自动实现的trait name
    let name = parse_struct_name(&s);
    //实现#[derive(HelloWorld)]， 自动设置并返回TokenStream
    let output = format!(r#"impl HelloWorld for {0} {{
        fn hello() {{ println!(" {0} says hello "); }}
    }}"#, name);

    TokenStream::from_str(&output).unwrap()
}

fn parse_struct_name(s: &str) -> String {
    let r = Regex::new(r"(?:struct\s+)([\w\d_]+)").unwrap();
    let caps = r.captures(s).unwrap();
    caps[1].to_string()
}