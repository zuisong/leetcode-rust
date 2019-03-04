/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
 *
 * https://leetcode-cn.com/problems/invert-binary-tree/description/
 *
 * algorithms
 * Easy (66.54%)
 * Total Accepted:    10.7K
 * Total Submissions: 16.1K
 * Testcase Example:  '[4,2,7,1,3,6,9]'
 *
 * 翻转一棵二叉树。
 *
 * 示例：
 *
 * 输入：
 *
 * ⁠    4
 * ⁠  /   \
 * ⁠ 2     7
 * ⁠/ \   / \
 * 1   3 6   9
 *
 * 输出：
 *
 * ⁠    4
 * ⁠  /   \
 * ⁠ 7     2
 * ⁠/ \   / \
 * 9   6 3   1
 *
 * 备注:
 * 这个问题是受到 Max Howell 的 原问题 启发的 ：
 *
 * 谷歌：我们90％的工程师使用您编写的软件(Homebrew)，但是您却无法在面试时在白板上写出翻转二叉树这道题，这太糟糕了。
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
use std::mem::swap;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree1(&root);
        root
    }

    fn invert_tree1(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let p = node.as_ptr();
            unsafe {
                swap(&mut (*p).left, &mut (*p).right);
            }
            Self::invert_tree1(&node.clone().borrow().left);
            Self::invert_tree1(&node.clone().borrow().right);
        };
    }
}


fn main() {
    let mut root = TreeNode::new(10);
    root.left = Option::Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.right = Option::Some(Rc::new(RefCell::new(TreeNode::new(11))));

    let root = Solution::invert_tree(Option::Some(Rc::new(RefCell::new(root))));

    println!("{:#?}", root);
}

struct Solution {}