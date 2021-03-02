/*

根据一棵树的中序遍历与后序遍历构造二叉树。

注意:
你可以假设树中没有重复的元素。

例如，给出

中序遍历 inorder = [9,3,15,20,7]
后序遍历 postorder = [9,15,7,20,3]
返回如下的二叉树：

    3
   / \
  9  20
    /  \
   15   7
通过次数91,231提交次数128,025

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

use std::borrow::Borrow;
use std::cell::RefCell;
//
use std::rc::Rc;

fn main() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let tree = Solution::build_tree(inorder, postorder);

    dbg!(&tree);
}

struct Solution {}

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

// --
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(inorder.borrow(), postorder.borrow())
    }

    pub fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() != postorder.len() {
            unreachable!();
        }
        if inorder.len() == 0 {
            return None;
        }

        // println!("{:?}", inorder);
        // println!("{:?}", postorder);

        let top_node_val = postorder[postorder.len() - 1];

        let mid_idx = inorder
            .iter()
            .enumerate()
            .find(|(_, val)| **val == top_node_val)
            .unwrap()
            .0;

        let l_inorder = &inorder[0..mid_idx];
        let r_inorder = &inorder[(mid_idx + 1)..inorder.len()];

        let l_postorder = &postorder[0..mid_idx];
        let r_postorder = &postorder[(mid_idx)..(inorder.len() - 1)];

        let left_node = Self::helper(l_inorder, l_postorder);
        let right_node = Self::helper(r_inorder, r_postorder);

        let tree = TreeNode {
            val: top_node_val,

            left: left_node,
            right: right_node,
        };

        return Some(Rc::new(RefCell::new(tree)));
    }
}
