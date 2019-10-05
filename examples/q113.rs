/*
给定一个二叉树和一个目标和，找到所有从根节点到叶子节点路径总和等于给定目标和的路径。

说明: 叶子节点是指没有子节点的节点。

示例:
给定如下二叉树，以及目标和 sum = 22，

              5
             / \
            4   8
           /   / \
          11  13  4
         /  \    / \
        7    2  5   1
返回:

[
   [5,4,11,2],
   [5,8,4,5]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/path-sum-ii
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        pub fn path_sum(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Option<Vec<Vec<i32>>> {
            match node {
                None => None,

                Some(n) => {
                    let node = n.borrow();
                    match (&node.left, &node.right, &node.val) {
                        (None, None, &v) => {
                            if v == sum {
                                return Some(vec![vec![sum]]);
                            } else {
                                return None;
                            }
                        }
                        _ => {}
                    }

                    let l_res = path_sum(&node.left, sum - node.val);
                    let r_res = path_sum(&node.right, sum - node.val);
                    let v = match (l_res, r_res) {
                        (Some(mut v1), Some(mut v2)) => {
                            v1.append(&mut v2);
                            Some(v1)
                        }
                        (a, b) => a.or(b),
                    };

                    if v.is_none() {
                        None
                    } else {
                        let mut res = v.unwrap();
                        res.iter_mut().for_each(|v| v.push(node.val));

                        Some(res)
                    }
                }
            }
        }
        let result = path_sum(&root, sum);
        result
            .map(|mut v| {
                v.iter_mut().for_each(|t| t.reverse());
                v
            })
            .unwrap_or(vec![])
    }
}

struct Solution {}

fn main() {}
