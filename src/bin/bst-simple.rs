#![feature(box_syntax)]

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use leetcode_rust::bst::BinarySearchTree;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            parent: None,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SimpleBinarySearchTree {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl SimpleBinarySearchTree {
    fn new() -> Self {
        SimpleBinarySearchTree { root: None }
    }
}

impl BinarySearchTree for SimpleBinarySearchTree {
    fn add(&mut self, item: i32) -> Result<(), &str> {
        let mut node: &mut _ = &mut self.root;

        loop {
            match node {
                None => {
                    std::mem::replace(node, Some(Rc::new(RefCell::new(TreeNode::new(item)))));
                    return Ok(());
                }
                Some(n) => {
                    let ordering = n.borrow().val.cmp(&item);
                    node = match ordering {
                        Ordering::Less => unsafe { &mut (*n.as_ptr()).right },
                        Ordering::Greater => unsafe { &mut (*n.as_ptr()).left },
                        Ordering::Equal => {
                            return Err("不能插入一样的值2");
                        }
                    }
                }
            }
        }
    }

    fn remove(&mut self, item: i32) -> bool {
        fn remove_node(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            let n = node.as_mut().unwrap();

            if n.borrow_mut().left.is_none() {
                let right = std::mem::replace(&mut n.borrow_mut().right, None);
                std::mem::replace(node, right);
            } else if n.borrow_mut().right.is_none() {
                let left = std::mem::replace(&mut n.borrow_mut().left, None);
                std::mem::replace(node, left);
            } else {
                let i = n.borrow().left.as_ref().unwrap().borrow().val;
                n.borrow_mut().val = i;
                remove_node(&mut n.borrow_mut().left);
            }
        }

        fn remove(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            match node {
                Some(n) => {
                    let ordering = n.borrow().val.cmp(&val);
                    match ordering {
                        Ordering::Less => remove(&mut n.borrow_mut().right, val),
                        Ordering::Greater => remove(&mut n.borrow_mut().left, val),
                        Ordering::Equal => {
                            remove_node(node);
                            true
                        }
                    }
                }

                None => false,
            }
        }
        remove(&mut self.root, item)
    }

    fn find(&self, item: i32) -> bool {
        fn find(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            match node {
                Some(n) => match n.borrow().val.cmp(&val) {
                    Ordering::Less => find(&n.borrow().right, val),
                    Ordering::Greater => find(&n.borrow().left, val),
                    Ordering::Equal => true,
                },
                None => false,
            }
        }

        let node: &Option<Rc<RefCell<TreeNode>>> = &self.root;
        find(node, item)
    }
}

fn main() {
    let mut tree: Box<dyn BinarySearchTree> = box SimpleBinarySearchTree::new();
    tree.add(2).unwrap();
    tree.add(1).unwrap();
    tree.add(3).unwrap();
    assert_eq!(true, tree.find(2));

    dbg!(&tree);

    tree.remove(2);
    dbg!(&tree);
    assert_eq!(false, tree.find(2));

    tree.add(2);
    dbg!(&tree);
    assert_eq!(true, tree.find(2));
}

#[cfg(test)]
mod tests {
    use leetcode_rust::bst::BinarySearchTree;
    use test_case::test_case;

    use crate::SimpleBinarySearchTree;

    #[test_case(0, 0)]
    #[test_case(1, 1)]
    #[test_case(2, 1)]
    #[test_case(3, 2)]
    #[test_case(4, 3)]
    #[test_case(5, 5)]
    #[test_case(6, 8)]
    #[test_case(7, 13)]
    #[test_case(8, 21)]
    fn fibonacci_test(input: u32, expected: u32) {
        let _a = 0;
        fn fibonacci(n: u32) -> u32 {
            if n >= 30 {
                return 0;
            }

            match n {
                0 => 0,
                1 => 1,
                _ => fibonacci(n - 1) + fibonacci(n - 2),
            }
        }
        println!("{} --> {}", input, fibonacci(input));

        assert_eq!(expected, fibonacci(input))
    }

    #[test]
    fn test() {
        let mut tree: Box<dyn BinarySearchTree> = box SimpleBinarySearchTree::new();
        assert_eq!(false, tree.find(1));
        assert_eq!(false, tree.find(2));
        assert_eq!(false, tree.find(3));
        assert_eq!(false, tree.find(0));
        tree.add(1).unwrap();
        assert_eq!(true, tree.find(1));
        tree.add(2).unwrap();
        assert_eq!(true, tree.find(2));
        tree.add(3).unwrap();
        assert_eq!(true, tree.find(3));

        assert_eq!(true, tree.find(1));
        assert_eq!(true, tree.find(2));
        assert_eq!(true, tree.find(3));

        tree.remove(1);
        assert_eq!(false, tree.find(1));
        tree.remove(2);
        assert_eq!(false, tree.find(2));
        tree.remove(3);
        assert_eq!(false, tree.find(3));

        assert_eq!(false, tree.find(0));
    }
}
