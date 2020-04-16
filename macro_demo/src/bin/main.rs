#[macro_use]
extern crate macro_demo;
#[macro_use]
extern crate log;
use env_logger;
use log::{debug, info};

use macro_demo::helped;
use macro_demo::unpub_fn_call;

use std::ops::Add;


macro_rules! add {
    ($one:literal, $two:literal) => {
        String::from($one).add($two)
    };
}

fn main() {
    env_logger::init();
    println!("Hello macro!",);

    // expr example 使用全限定名，会从指定的路径下查找宏
    println!("1 + 5 = {}", macro_demo::add!(1, 5));
    // println!("1 + 1 = {}", add!(1));
    //literal example 没有指定全限定名，默认会在 本文本范围 内查找宏，找不到再到路径下查找宏。
    println!("{}", add!("aaa", "bbb"));

    // // path exmaple
    // path_example!()

    // /// ty example
    // ty_example!(1_i32, )


    // -------------------   宏覆盖 -------------------------
    info!("info aaaa");

    macro_rules! info {
        ($s:literal) => {
            println!("self info: {}", $s);
        };
    }

    info!("info2 bbb");

    // ident example
    ident_example!(hgf);
    hgf();


    // -------------------   宏 健康问题 -------------------------
    //  宏a和b 同属于一个crate x，且a 直接使用名字调用了b，但是其他crate 使用x时，仅引用了宏a，则在其他crate 调用宏a的时候可能出现问题（正确做法宏a使用时宏b时，需要使用$crate::b!() 调用b宏 ）。
    fn unit() {
        helped!();
        println!("called helped!", );
    }
    unit();

    // ---------------------- 宏使用$crate 调用crate 内部 非公开方法 -------------------------

    // unpub_fn_call!();   // 错误的， unpub_fn_call宏调用crate 内部 非公开方法， 导致外部crate 在调用unpub_fn_call宏找不到方法。

}
