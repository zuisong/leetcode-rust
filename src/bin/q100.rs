//给定两个二叉树，编写一个函数来检验它们是否相同。
//
// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
//
// 示例 1:
//
// 输入:       1         1
//          / \       / \
//         2   3     2   3
//
//        [1,2,3],   [1,2,3]
//
//输出: true
//
// 示例 2:
//
// 输入:      1          1
//          /           \
//         2             2
//
//        [1,2],     [1,null,2]
//
//输出: false
//
//
// 示例 3:
//
// 输入:       1         1
//          / \       / \
//         2   1     1   2
//
//        [1,2,1],   [1,1,2]
//
//输出: false
//
//

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_same_tree2(
            p: &Option<Rc<RefCell<TreeNode>>>,
            q: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            return match (p, q) {
                (None, None) => true,
                (Some(pn), Some(qn)) => {
                    is_same_tree2(&pn.borrow().left, &qn.borrow().left)
                        && is_same_tree2(&pn.borrow().right, &qn.borrow().right)
                        && pn.borrow().val == qn.borrow().val
                }
                (_, _) => false,
            };
        }
        is_same_tree2(&p, &q)
    }
}

struct Solution {}

fn main() {}
