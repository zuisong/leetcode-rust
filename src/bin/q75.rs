//给定一个包含红色、白色和蓝色，一共 n 个元素的数组，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
//
// 此题中，我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
//
// 注意:
//不能使用代码库中的排序函数来解决这道题。
//
// 示例:
//
// 输入: [2,0,2,1,1,0]
//输出: [0,0,1,1,2,2]
//
// 进阶：
//
//
// 一个直观的解决方案是使用计数排序的两趟扫描算法。
// 首先，迭代计算出0、1 和 2 元素的个数，然后按照0、1、2的排序，重写当前数组。
// 你能想出一个仅使用常数空间的一趟扫描算法吗？
//
// Related Topics 排序 数组 双指针



//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }

        let mut idx_0 = 0;
        let mut idx_2 = nums.len() - 1;
        let mut idx = 0;
        while idx <= idx_2 {
            match nums[idx] {
                0 => {
                    nums[idx] = 1;
                    idx += 1;

                    nums[idx_0] = 0;
                    idx_0 += 1;
                }
                1 => {
                    idx += 1;
                }
                2 => {
                    match nums[idx_2] {
                        0 => {
                            nums[idx_2] = 2;
                            idx_2 -= 1;

                            nums[idx] = 1;

                            nums[idx_0] = 0;
                            idx_0 += 1;


                            idx += 1;
                        }
                        1 => {
                            nums[idx_2] = 2;
                            idx_2 -= 1;
                            nums[idx] = 1;
                            idx += 1;
                        }
                        2 => {
                            if idx_2 <= 0 {
                                break;
                            }
                            idx_2 -= 1;
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
    }

    pub fn sort_colors1(nums: &mut Vec<i32>) {
        let mut count_0 = 0;
        let mut count_1 = 0;
        let mut count_2 = 0;

        for x in nums.iter() {
            match x {
                0 => count_0 += 1,
                1 => count_1 += 1,
                2 => count_2 += 1,
                _ => unreachable!()
            }
        }

        for idx in 0..count_0 {
            nums[idx] = 0;
        }

        for idx in count_0..(count_1 + count_0) {
            nums[idx] = 1;
        }

        for idx in (nums.len() - count_2)..nums.len() {
            nums[idx] = 2;
        }
    }
}

//leetcode submit region end(Prohibit modification and deletion)
struct Solution {}

fn main() {
    let mut nums = vec![2, 0, 1];
    nums = vec![2, 0, 2, 1, 1, 0];
//    nums = vec![2];

    let mut res = nums.clone();

    res.sort();

    Solution::sort_colors(&mut nums);
    dbg!(&nums);
}