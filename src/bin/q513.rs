//给定一个二叉树，在树的最后一行找到最左边的值。
//
// 示例 1:
//
//
//输入:
//
//    2
//   / \
//  1   3
//
//输出:
//1
//
//
//
//
// 示例 2:
//
//
//输入:
//
//        1
//       / \
//      2   3
//     /   / \
//    4   5   6
//       /
//      7
//
//输出:
//7
//
//
//
//
// 注意: 您可以假设树（即给定的根节点）不为 NULL。
// Related Topics 树 深度优先搜索 广度优先搜索

//leetcode submit region begin(Prohibit modification and deletion)
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

use std::cell::RefCell;
use std::rc::Rc;

pub fn find_bottom_left_value(
    node: &Option<Rc<RefCell<TreeNode>>>,
    level: i32,
) -> Option<(i32, i32)> {
    return match node {
        Some(n) => {
            let left = find_bottom_left_value(&n.borrow().left, level + 1);
            let right = find_bottom_left_value(&n.borrow().right, level + 1);

            let res = match (left, right) {
                (Some(l), Some(r)) =>
                    if l.1 >= r.1 {
                        l
                    } else {
                        r
                    },
                (Some(l), None) => l,
                (None, Some(r)) => r,
                (None, None) => (n.borrow().val, level),
            };
            Some(res)
        }
        None => None,
    };
}

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let option = find_bottom_left_value(&root, 0);
        return option.unwrap().0;
    }
}
//leetcode submit region end(Prohibit modification and deletion)

fn main() {}
