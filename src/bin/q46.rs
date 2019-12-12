//给定一个没有重复数字的序列，返回其所有可能的全排列。
//
// 示例:
//
// 输入: [1,2,3]
//输出:
//[
//  [1,2,3],
//  [1,3,2],
//  [2,1,3],
//  [2,3,1],
//  [3,1,2],
//  [3,2,1]
//]
// Related Topics 回溯算法

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>) {
            if temp.len() == nums.len() {
                res.push(temp.clone());
                return;
            }

            for num in nums {
                if !temp.contains(num) {
                    temp.push(*num);
                    dfs(res, temp, nums);
                    temp.pop();
                }
            }
        }
        let mut result = Vec::new();

        dfs(&mut result, &mut Vec::new(), &nums);
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

fn main() {
    let solution = Solution::permute(vec![1, 2, 3]);
    println!("{:?}", solution);
}

struct Solution {}
