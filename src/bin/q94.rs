//给定一个二叉树，返回它的中序 遍历。
//
// 示例:
//
// 输入: [1,null,2,3]
//   1
//    \
//     2
//    /
//   3
//
//输出: [1,3,2]
//
// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
// Related Topics 栈 树 哈希表

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

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn f(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            match node {
                Some(n) => {
                    f(&n.borrow().left, res);
                    res.push(n.borrow().val);
                    f(&n.borrow().right, res);
                }
                None => {}
            }
        }
        let mut res = vec![];
        f(&root, &mut res);
        return res;
    }
}

//leetcode submit region end(Prohibit modification and deletion)
struct Solution {}

fn main() {}
