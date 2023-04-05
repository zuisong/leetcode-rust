/*
给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。

高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。



示例 1：


输入：nums = [-10,-3,0,5,9]
输出：[0,-3,9,-10,null,5]
解释：[0,-10,5,null,-3,null,9] 也将被视为正确答案：

示例 2：


输入：nums = [1,3]
输出：[3,1]
解释：[1,3] 和 [3,1] 都是高度平衡二叉搜索树。


提示：

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums 按 严格递增 顺序排列
通过次数141,056提交次数188,622

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::helper(nums.as_slice());
    }
    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();

        if len == 0 {
            return None;
        }

        let val = nums[len / 2];
        let left_node = Self::helper(&nums[0..len / 2]);
        let right_node = Self::helper(&nums[(len / 2 + 1)..len]);
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: left_node,
            right: right_node,
        })))
    }
}

//
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

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];
    let bst = Solution::sorted_array_to_bst(nums);
    dbg!(&bst);
}
