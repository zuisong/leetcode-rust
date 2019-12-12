pub trait BinarySearchTree: std::fmt::Debug {
    fn add(&mut self, item: i32) -> Result<(), &str>;
    fn remove(&mut self, item: i32) -> bool;
    fn find(&self, item: i32) -> bool;
}
