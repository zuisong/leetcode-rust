#![feature(box_syntax)]

use log::Level::*;

pub mod bst;

#[allow(unused_must_use)]
pub fn init_logger() {
    simple_logger::init_with_level(Debug);
}
