/*
 * @lc app=leetcode.cn id=563 lang=rust
 *
 * [563] 二叉树的坡度
 *
 * https://leetcode-cn.com/problems/binary-tree-tilt/description/
 *
 * algorithms
 * Easy (47.23%)
 * Total Accepted:    2.5K
 * Total Submissions: 5.3K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个二叉树，计算整个树的坡度。
 *
 * 一个树的节点的坡度定义即为，该节点左子树的结点之和和右子树结点之和的差的绝对值。空结点的的坡度是0。
 *
 * 整个树的坡度就是其所有节点的坡度之和。
 *
 * 示例:
 *
 *
 * 输入:
 *         1
 *       /   \
 *      2     3
 * 输出: 1
 * 解释:
 * 结点的坡度 2 : 0
 * 结点的坡度 3 : 0
 * 结点的坡度 1 : |2-3| = 1
 * 树的坡度 : 0 + 0 + 1 = 1
 *
 *
 * 注意:
 *
 *
 * 任何子树的结点的和不会超过32位整数的范围。
 * 坡度的值不会超过32位整数的范围。
 *
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let t = Self::v_node(&root);
        t.1
    }

    fn v_node(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            Some(node) => {
                let x = node.borrow();
                let (l_sum, l_tilt) = Self::v_node(&x.left);
                let (r_sum, r_tilt) = Self::v_node(&x.right);
                (
                    l_sum + r_sum + x.val,
                    l_tilt + r_tilt + (l_sum - r_sum).abs(),
                )
            }
            _ => (0, 0),
        }
    }
}

struct Solution {}

fn main() {}
