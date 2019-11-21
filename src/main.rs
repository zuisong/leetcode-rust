use log::*;

fn init_logger() {
    env_logger::Builder::default()
        .filter_level(LevelFilter::Debug)
        .init();
}

fn main() {
    init_logger();

    let d = f1();

    info!("23456");

    info!("Hello World, {}", d(1000));
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
