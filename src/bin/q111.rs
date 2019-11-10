/*
给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

说明: 叶子节点是指没有子节点的节点。

示例:

给定二叉树 [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回它的最小深度  2.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-depth-of-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::cell::Cell;
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn min_depth(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32, min: &mut Cell<i32>) {
            if node.is_none() {
                return;
            }
            let node = node.as_ref().unwrap();

            if depth > min.get() {
                return;
            }

            match (&node.borrow().left, &node.borrow().right) {
                (None, None) => {
                    min.set(depth);
                }
                (left, right) => {
                    min_depth(left, depth + 1, min);
                    min_depth(right, depth + 1, min);
                }
            }
        }
        let mut cell = Cell::new(10000);
        match root {
            None => 0,
            _ => {
                min_depth(&root, 1, &mut cell);
                cell.get()
            }
        }
    }
}

struct Solution {}

fn main() {}
