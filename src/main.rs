use log::*;

use leetcode_rust::init_logger;

fn main() {
    init_logger();

    let d = f1();

    info!("23456");

    let d1: i32 = d(1000);
    info!("Hello World, {}", d1);
}

fn f1() -> Box<dyn Fn(i32) -> i32> {
    let s = "151545";
    let x1: Box<dyn Fn(i32) -> i32> = Box::from(move |x: i32| {
        println!("{}", &s);
        x + 1
    });
    println!("{}", s);

    x1
}
