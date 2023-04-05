/*
给定两个大小为 m 和 n 的正序（从小到大）数组nums1 和nums2。请你找出并返回这两个正序数组的中位数。

进阶：你能设计一个时间复杂度为 O(log (m+n)) 的算法解决此问题吗？



示例 1：

输入：nums1 = [1,3], nums2 = [2]
输出：2.00000
解释：合并数组 = [1,2,3] ，中位数 2
示例 2：

输入：nums1 = [1,2], nums2 = [3,4]
输出：2.50000
解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
示例 3：

输入：nums1 = [0,0], nums2 = [0,0]
输出：0.00000
示例 4：

输入：nums1 = [], nums2 = [1]
输出：1.00000
示例 5：

输入：nums1 = [2], nums2 = []
输出：2.00000


提示：

nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000
-106 <= nums1[i], nums2[i] <= 106
通过次数314,972提交次数799,604

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

fn main() {
    let nums1 = vec![1];
    let nums2 = vec![2, 3, 4, 5, 6];
    let ans = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("{:?}", ans);
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn find_median_sorted_arrays2(
            top_k: usize,
            nums1: &Vec<i32>,
            idx1: usize,
            nums2: &Vec<i32>,
            idx2: usize,
        ) -> i32 {
            if idx1 >= nums1.len() {
                return nums2[top_k - 1 + idx2];
            }

            if idx2 >= nums2.len() {
                return nums1[top_k - 1 + idx1];
            }

            // 折半删除法
            if top_k == 1 {
                return nums1[idx1].min(nums2[idx2]);
            }

            let half_count = top_k / 2;

            if nums1[half_count + idx1 - 1] > nums2[half_count + idx2 - 1] {
                // 删掉 left2 的一半
                find_median_sorted_arrays2(
                    top_k - half_count,
                    nums1,
                    idx1,
                    nums2,
                    idx2 + half_count,
                )
            } else {
                // 删掉 left1 的一半
                find_median_sorted_arrays2(
                    top_k - half_count,
                    nums1,
                    idx1 + half_count,
                    nums2,
                    idx2,
                )
            }
        }
        let total_size = nums1.len() + nums2.len();
        if total_size % 2 == 1 {
            return find_median_sorted_arrays2(total_size / 2 + 1, &nums1, 0, &nums2, 0) as f64;
        } else if total_size % 2 == 0 {
            return (find_median_sorted_arrays2(total_size / 2, &nums1, 0, &nums2, 0) as f64
                + find_median_sorted_arrays2(total_size / 2 + 1, &nums1, 0, &nums2, 0) as f64)
                / 2.0;
        }
        0.0
    }
}
