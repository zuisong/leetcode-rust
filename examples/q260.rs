/*
给定一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。

示例 :

输入: [1,2,1,3,2,5]
输出: [3,5]
注意：

结果输出的顺序并不重要，对于上面的例子， [5, 3] 也是正确答案。
你的算法应该具有线性时间复杂度。你能否仅使用常数空间复杂度来实现？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/single-number-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_res = nums.iter().fold(0, |a, b| a ^ *b);
        let mut n = 0;
        for i in 0..32 {
            if xor_res & (1 << i) != 0 {
                n = 1 << i;
                break;
            }
        }

        let mut a = 0;
        let mut b = 0;

        for num in nums.into_iter() {
            if num & n == 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }
        vec![a, b]
    }
}

fn main() {
    let res = Solution::single_number(vec![3, 5, 1, 3, 2, 5]);
    dbg!(res);
}

struct Solution {}