#![feature(test)]
//#![allow(unused_imports)]

extern crate test;

use test::Bencher;

#[bench]
fn main1(b: &mut Bencher) {
    b.iter(|| main())
}

fn main() {
    let d = f1();

    println!("23456");

    println!("Hello World, {}", d(1000));
}

fn f1() -> Box<dyn Fn(i32) -> i32> {
    let s = "151545";
    let x1: Box<dyn Fn(i32) -> i32> = Box::from(move |x: i32| {
        println!("{}", &s);
        return x + 1;
    });
    println!("{}", s);

    return x1;
}
