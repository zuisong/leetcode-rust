/*

给定一个二叉树，判断其是否是一个有效的二叉搜索树。

假设一个二叉搜索树具有如下特征：

节点的左子树只包含小于当前节点的数。
节点的右子树只包含大于当前节点的数。
所有左子树和右子树自身必须也是二叉搜索树。
示例 1:

输入:
    2
   / \
  1   3
输出: true
示例 2:

输入:
    5
   / \
  1   4
/ \
3   6
输出: false
解释: 输入为: [5,1,4,null,null,3,6]。
根节点的值为 5 ，但是其右子节点值为 4 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/validate-binary-search-tree
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid_bst2(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let node = n.borrow();
                    let left = &node.left;
                    let right = &node.right;

                    node.val as i64 > min
                        && (node.val as i64) < max
                        && (left.is_none() || node.val > (left.as_ref().unwrap().borrow().val))
                        && is_valid_bst2(left, min, node.val as i64)
                        && (right.is_none() || node.val < (right.as_ref().unwrap().borrow().val))
                        && is_valid_bst2(right, node.val as i64, max)
                }
            }
        }
        return is_valid_bst2(&root, i64::min_value(), i64::max_value());
    }
}

struct Solution {}

fn main() {}
