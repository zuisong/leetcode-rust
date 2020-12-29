#![feature(box_syntax)]

pub mod bst;

#[allow(unused_must_use)]
pub fn init_logger() {
    simple_logger::SimpleLogger::new().init();
}
