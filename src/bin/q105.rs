use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

/**

根据一棵树的前序遍历与中序遍历构造二叉树。

注意:
你可以假设树中没有重复的元素。

例如，给出

前序遍历 preorder = [3,9,20,15,7]
中序遍历 inorder = [9,3,15,20,7]
返回如下的二叉树：

    3
   / \
  9  20
    /  \
   15   7
通过次数156,027提交次数225,570

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
fn main() {
    /*

        3
       / \
      9  20
     /  /  \
    1  15   7

         */
    let preorder = vec![3, 9, 1, 20, 15, 7];
    let inorder = vec![1, 9, 3, 15, 20, 7];

    let tree = Solution::build_tree(preorder, inorder);

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

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::helper(preorder.borrow(), inorder.borrow());
    }

    pub fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() != inorder.len() {
            unreachable!();
        }

        if preorder.len() == 0 {
            return None;
        }
        // println!("preorder = {:?}", preorder);
        // println!("inorder  = {:?}", inorder);
        let first_val = *preorder.first().unwrap();

        let mid = inorder
            .iter()
            .enumerate()
            .find(|(_, val)| **val == first_val)
            .unwrap()
            .0;

        let left_preorder = &preorder[1..(mid + 1)];
        let right_preorder = &preorder[(mid + 1)..preorder.len()];

        let left_inorder = &inorder[0..mid];
        let right_inorder = &inorder[(mid + 1)..inorder.len()];

        let left_node = Self::helper(left_preorder, left_inorder);
        let right_node = Self::helper(right_preorder, right_inorder);

        let mut tree_node = TreeNode::new(first_val);
        tree_node.left = left_node;
        tree_node.right = right_node;
        return Some(Rc::new(RefCell::new(tree_node)));
    }
}
