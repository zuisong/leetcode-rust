/*
给定一个二叉树，返回其节点值的锯齿形层次遍历。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。

例如：
给定二叉树 [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回锯齿形层次遍历如下：

[
  [3],
  [20,9],
  [15,7]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-zigzag-level-order-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut v = VecDeque::new();
        v.push_back(root);
        v.push_back(None);
        let mut res = vec![];
        let mut temp = vec![];
        let mut temp_vec = VecDeque::new();
        let mut left2right = true;
        loop {
            match v.pop_front().unwrap() {
                Some(n) => {
                    temp.push(n.borrow().val);
                    let left = std::mem::replace(&mut n.borrow_mut().left, None);
                    let right = std::mem::replace(&mut n.borrow_mut().right, None);
                    if left2right {
                        if left.is_some() {
                            temp_vec.push_back(left);
                        }

                        if right.is_some() {
                            temp_vec.push_back(right);
                        }
                    } else {
                        if right.is_some() {
                            temp_vec.push_back(right);
                        }
                        if left.is_some() {
                            temp_vec.push_back(left);
                        }
                    }
                }
                None => {
                    res.push(std::mem::replace(&mut temp, vec![]));
                    if temp_vec.is_empty() {
                        break;
                    }
                    while let Some(node) = temp_vec.pop_back() {
                        v.push_back(node);
                    }

                    left2right = !left2right;

                    v.push_back(None)
                }
            }
        }
        res
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    let mut l1 = TreeNode::new(2);
    l1.left = Option::Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let mut r1 = TreeNode::new(3);
    r1.right = Option::Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.left = Option::Some(Rc::new(RefCell::new(l1)));
    root.right = Option::Some(Rc::new(RefCell::new(r1)));

    let root = Option::Some(Rc::new(RefCell::new(root)));
    let result = Solution::zigzag_level_order(root);

    println!("{:?}", result);
}

struct Solution {}
