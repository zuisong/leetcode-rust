//Definition for a binary tree node.
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

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let val = node.borrow().val;
            let mut max_l = Cell::new(0);
            let r = Option::Some(node);

            Self::get_max_l(&r, val, &mut max_l);
            return max_l.get();
        };
        0
    }

    fn get_max_l(r: &Option<Rc<RefCell<TreeNode>>>, val: i32, max_l: &mut Cell<i32>) -> i32 {
        return match r {
            None => 0,
            Some(r) => {
                let node = r.borrow();
                let val2 = node.val;
                let left = Solution::get_max_l(&node.left, val2, max_l);
                let right = Solution::get_max_l(&node.right, val2, max_l);
                // 路径长度为节点数减1所以此处不加1
                max_l.set(max_l.get().max(left + right));
                // 和父节点值相同才返回以当前节点所能构成的最长通知路径长度, 否则返回0
                if val2 == val {
                    return left.max(right) + 1;
                } else {
                    return 0;
                }
            }
        };
    }
}

fn main() {
    let n = TreeNode::new(5);

    let node = Option::Some(Rc::new(RefCell::new(n)));
    let result = Solution::longest_univalue_path(node);

    println!("{:?}", result);
}
