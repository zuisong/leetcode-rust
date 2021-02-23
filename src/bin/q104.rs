use std::cell::RefCell;
use std::rc::Rc;

/**

给定一个二叉树，找出其最大深度。

二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。

说明: 叶子节点是指没有子节点的节点。

示例：
给定二叉树 [3,9,20,null,null,15,7]，

    3
   / \
  9  20
    /  \
   15   7
返回它的最大深度 3 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-depth-of-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

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

struct Solution {}

fn main() {}

///
///
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::max_depth2(&root);
    }

    pub fn max_depth2(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            Some(n) => {
                let left = &n.borrow().left;
                let left_depth = Self::max_depth2(left);

                let right = &n.borrow().right;
                let right_depth = Self::max_depth2(right);

                std::cmp::max(left_depth, right_depth) + 1
            }
            None => 0
        }
    }
}
