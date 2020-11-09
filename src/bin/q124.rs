/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 *
 * https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/description/
 *
 * algorithms
 * Hard (33.54%)
 * Total Accepted:    4.9K
 * Total Submissions: 14.3K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个非空二叉树，返回其最大路径和。
 *
 * 本题中，路径被定义为一条从树中任意节点出发，达到任意节点的序列。该路径至少包含一个节点，且不一定经过根节点。
 *
 * 示例 1:
 *
 * 输入: [1,2,3]
 *
 *       1
 *      / \
 *     2   3
 *
 * 输出: 6
 *
 *
 * 示例 2:
 *
 * 输入: [-10,9,20,null,null,15,7]
 *
 * -10
 * / \
 * 9  20
 * /  \
 * 15   7
 *
 * 输出: 42
 *
 */
use std::cell::{Cell, RefCell};
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_path_sum1(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut Cell<i32>) -> i32 {
            match node {
                Some(n) => {
                    let left_sum = max_path_sum1(&n.borrow().left, max_sum);
                    let right_sum = max_path_sum1(&n.borrow().right, max_sum);

                    max_sum.set(max_sum.get().max(left_sum + right_sum + n.borrow().val));
                    // 小于0 的时候直接舍弃掉
                    0.max(left_sum.max(right_sum) + n.borrow().val)
                }
                None => 0,
            }
        }
        let mut cell = Cell::new(0x7f_ff_ff_ff * -1);
        max_path_sum1(&root, &mut cell);
        cell.get()
    }
}

fn main() {}

struct Solution {}
