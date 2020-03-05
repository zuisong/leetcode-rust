#![feature(box_syntax)]

use log::Level::{Debug, Info};
use log::LevelFilter;

pub mod bst;

pub fn init_logger() {
    simple_logger::init_with_level(Debug);
}
