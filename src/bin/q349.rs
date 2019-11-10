//给定两个数组，编写一个函数来计算它们的交集。
//
// 示例 1:
//
// 输入: nums1 = [1,2,2,1], nums2 = [2,2]
//输出: [2]
//
//
// 示例 2:
//
// 输入: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
//输出: [9,4]
//
// 说明:
//
//
// 输出结果中的每个元素一定是唯一的。
// 我们可以不考虑输出结果的顺序。
//
// Related Topics 排序 哈希表 双指针 二分查找

use std::collections::HashSet;
use std::iter::FromIterator;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        let set2: HashSet<i32> = HashSet::from_iter(nums2.into_iter());

        set2.into_iter().filter(|it| set1.contains(it)).collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

struct Solution;

fn main() {
    let result = Solution::intersection(vec![1, 2, 3], vec![2, 3, 4]);
    dbg!(result);
}
