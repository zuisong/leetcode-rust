/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
 *
 * https://leetcode-cn.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (45.42%)
 * Total Accepted:    25.1K
 * Total Submissions: 55K
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * 给定一个二叉树，检查它是否是镜像对称的。
 *
 * 例如，二叉树 [1,2,2,3,4,4,3] 是对称的。
 *
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠/ \ / \
 * 3  4 4  3
 *
 *
 * 但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:
 *
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠  \   \
 * ⁠  3    3
 *
 *
 * 说明:
 *
 * 如果你可以运用递归和迭代两种方法解决这个问题，会很加分。
 *
 */
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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn symmetric(
            node1: &Option<Rc<RefCell<TreeNode>>>,
            node2: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if node1.is_none() && node2.is_none() {
                return true;
            }
            if node1.is_none() || node2.is_none() {
                return false;
            }
            let node1_mut = node1.as_ref().unwrap().borrow();
            let node2_mut = node2.as_ref().unwrap().borrow();

            node1_mut.val == node2_mut.val
                && symmetric(&node1_mut.right, &node2_mut.left)
                && symmetric(&node1_mut.left, &node2_mut.right)
        }
        if root.is_none() { return true; }
        let root = root.as_ref().unwrap().borrow();
        symmetric(&root.left, &root.right)
    }
}

fn main() {
    Solution::is_symmetric(None);
}

struct Solution {}

