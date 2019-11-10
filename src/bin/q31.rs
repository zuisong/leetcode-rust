/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 *
 * https://leetcode-cn.com/problems/next-permutation/description/
 *
 * algorithms
 * Medium (29.96%)
 * Total Accepted:    8.9K
 * Total Submissions: 29.7K
 * Testcase Example:  '[1,2,3]'
 *
 * 实现获取下一个排列的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列。
 *
 * 如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
 *
 * 必须原地修改，只允许使用额外常数空间。
 *
 * 以下是一些例子，输入位于左侧列，其相应输出位于右侧列。
 * 1,2,3 → 1,3,2
 * 3,2,1 → 1,2,3
 * 1,1,5 → 1,5,1
 *
 */
// 从后往前找到第一个递减的数， 在后面找到一个刚比他大一点的数替换，把后面的数弄成顺序排列

//   [4, 5, 3, 2]   -> [5,2,3,4]   找到第一个递减的 也就是 4
// 然后在后面的数中找到一个刚比他大的数  也就是 5
// 和 4 互换位置  然后  把   4 3 2 弄成顺序排列   5 2 3 4
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut b = true;
        let mut idx: usize = 0;
        let l = nums.len();
        for i in (1..l).rev() {
            if nums[i - 1] < nums[i] {
                b = false;
                idx = i - 1;
                break;
            }
        }

        if b {
            nums.reverse();
        } else {
            let mut i = l - 1;

            while nums[i] <= nums[idx] {
                i -= 1;
            }

            nums.swap(idx, i);

            let mut i = idx + 1;
            let mut j = l - 1;

            while i < j {
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
    }
}

fn main() {
    check(&mut vec![1, 2, 3]);
    check(&mut vec![3, 2, 1]);
    check(&mut vec![2, 3, 1]);
    check(&mut vec![1, 4, 3, 2]);
    check(&mut vec![1, 1, 5]);
    check(&mut vec![3, 2, 3, 4]);
    check(&mut vec![1, 2, 3, 4]);
}

fn check(v: &mut Vec<i32>) {
    print!("{:?}  -->  ", v);
    Solution::next_permutation(v);
    println!("{:?}", v);
}

struct Solution {}
