#![feature(box_syntax)]

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub trait BinarySearchTree: std::fmt::Debug {
    fn add(&mut self, item: i32) -> Result<(), &str>;
    fn remove(&mut self, item: i32) -> bool;
    fn find(&self, item: i32) -> bool;
}
