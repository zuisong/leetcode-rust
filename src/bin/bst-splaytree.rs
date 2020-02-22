//#![feature(box_syntax)]
//
//use std::cell::RefCell;
//use std::cmp::Ordering;
//use std::rc::Rc;
//
//use leetcode_rust::bst::*;
//
//// Definition for a binary tree node.
//#[derive(Debug, PartialEq, Eq)]
//pub struct TreeNode {
//    pub val: i32,
//    pub left: Option<Rc<RefCell<TreeNode>>>,
//    pub right: Option<Rc<RefCell<TreeNode>>>,
//    pub parent: Option<Rc<RefCell<TreeNode>>>,
//}
//
//impl TreeNode {
//    pub fn new(val: i32) -> Self {
//        TreeNode {
//            val,
//            left: None,
//            right: None,
//            parent: None,
//        }
//    }
//}
//
//#[derive(Debug, PartialEq, Eq)]
//struct SplayBinarySearchTree {
//    root: Option<Rc<RefCell<TreeNode>>>,
//}
//
//impl SplayBinarySearchTree {
//    fn new() -> Self {
//        SplayBinarySearchTree { root: None }
//    }
//}
//
//impl BinarySearchTree for SplayBinarySearchTree {
//    fn add(&mut self, item: i32) -> Result<(), &str> {
//        fn add(
//            node: &mut Option<Rc<RefCell<TreeNode>>>,
//            val: i32,
//            parent: Option<Rc<RefCell<TreeNode>>>,
//        ) -> Result<(), &str> {
//            match node {
//                None => {
//                    std::mem::replace(node, Some(Rc::new(RefCell::new(TreeNode::new(val)))));
//                    Ok(())
//                }
//                Some(n) => {
//                    let ordering = n.borrow().val.cmp(&val);
//                    match ordering {
//                        Ordering::Less => add(&mut n.borrow_mut().right, val, node.clone()),
//                        Ordering::Greater => add(&mut n.borrow_mut().left, val, node.clone()),
//                        Ordering::Equal => Err("不能插入一样的值"),
//                    }
//                }
//            }
//        }
//        return add(&mut self.root, item, None);
//    }
//
//    fn remove(&mut self, item: i32) -> bool {
//        fn remove(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
//            match node {
//                Some(n) => {
//                    let ordering = n.borrow().val.cmp(&val);
//                    match ordering {
//                        Ordering::Less => remove(&mut n.borrow_mut().right, val),
//                        Ordering::Greater => remove(&mut n.borrow_mut().left, val),
//                        Ordering::Equal => {
//                            fn remove_node(node: &mut Option<Rc<RefCell<TreeNode>>>) {
//                                let n = node.as_mut().unwrap();
//
//                                if n.borrow_mut().left.is_none() {
//                                    let mut right =
//                                        std::mem::replace(&mut n.borrow_mut().right, None);
//                                    std::mem::replace(node, right);
//                                } else if n.borrow_mut().right.is_none() {
//                                    let mut left =
//                                        std::mem::replace(&mut n.borrow_mut().left, None);
//                                    std::mem::replace(node, left);
//                                } else {
//                                    let i = n.borrow().left.as_ref().unwrap().borrow().val;
//                                    n.borrow_mut().val = i;
//                                    remove_node(&mut n.borrow_mut().left);
//                                }
//                            }
//                            remove_node(node);
//                            true
//                        }
//                    }
//                }
//
//                None => false,
//            }
//        }
//        remove(&mut self.root, item)
//    }
//
//    fn find<'a>(&'a self, item: i32) -> bool {
//        fn find<'a>(node: &'a Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
//            match node {
//                Some(n) => match n.borrow().val.cmp(&val) {
//                    Ordering::Less => find(&n.borrow().right, val),
//                    Ordering::Greater => find(&n.borrow().left, val),
//                    Ordering::Equal => true,
//                },
//                None => false,
//            }
//        }
//
//        let mut node: &Option<Rc<RefCell<TreeNode>>> = &self.root;
//        find(node, item)
//    }
//}
//
//fn main() {
//    let mut tree: Box<dyn BinarySearchTree> = box SplayBinarySearchTree::new();
//    tree.add(2).unwrap();
//    tree.add(1).unwrap();
//    tree.add(3).unwrap();
//    assert_eq!(true, tree.find(2));
//
//    dbg!(&tree);
//
//    tree.remove(2);
//    dbg!(&tree);
//    assert_eq!(false, tree.find(2));
//
//    tree.add(2);
//    dbg!(&tree);
//    assert_eq!(true, tree.find(2));
//}
//

fn main() {}
