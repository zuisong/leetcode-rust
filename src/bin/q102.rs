/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层次遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-level-order-traversal/description/
 *
 * algorithms
 * Medium (53.56%)
 * Total Accepted:    19.8K
 * Total Submissions: 36.5K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。
 *
 * 例如:
 * 给定二叉树: [3,9,20,null,null,15,7],
 *
 *    3
 *   / \
 *  9  20
 *    /  \
 *   15   7
 *
 *
 * 返回其层次遍历结果：
 *
 * [
 *  [3],
 *  [9,20],
 *  [15,7]
 * ]
 *
 *
 */
use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut v = VecDeque::new();
        v.push_back(root);
        v.push_back(None);
        let mut res = vec![];
        let mut temp = vec![];

        loop {
            match v.pop_front().unwrap() {
                Some(n) => {
                    temp.push(n.borrow().val);
                    let left = std::mem::replace(&mut n.borrow_mut().left, None);
                    let right = std::mem::replace(&mut n.borrow_mut().right, None);
                    if left.is_some() {
                        v.push_back(left);
                    }

                    if right.is_some() {
                        v.push_back(right);
                    }
                }
                None => {
                    res.push(std::mem::replace(&mut temp, vec![]));
                    if v.is_empty() {
                        break;
                    }
                    v.push_back(None)
                }
            }
        }
        res
    }
}

fn main() {
    let n = TreeNode::new(5);

    let node = Option::Some(Rc::new(RefCell::new(n)));
    let result = Solution::level_order(node);

    println!("{:?}", result);
}

struct Solution {}
