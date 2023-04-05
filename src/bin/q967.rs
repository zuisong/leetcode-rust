//    返回所有长度为 N 且满足其每两个连续位上的数字之间的差的绝对值为 K 的非负整数。
//
//    请注意，除了数字 0 本身之外，答案中的每个数字都不能有前导零。例如，01 因为有一个前导零，所以是无效的；但 0 是有效的。
//
//    你可以按任何顺序返回答案。
//
//
//
//    示例 1：
//
//    输入：N = 3, K = 7
//    输出：[181,292,707,818,929]
//    解释：注意，070 不是一个有效的数字，因为它有前导零。
//    示例 2：
//
//    输入：N = 2, K = 1
//    输出：[10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
//
//
//    提示：
//
//    1 <= N <= 9
//    0 <= K <= 9
//
pub struct Solution {}

impl Solution {
    fn dfs(deep: i32, val: i32, n: i32, k: i32, res: &mut Vec<i32>) {
        if deep == 0 {
            for i in 0..=9 {
                Solution::dfs(1, i, n, k, res)
            }
        } else if deep < n && val != 0 {
            let latest_num = val % 10;

            if 9 >= latest_num - k && latest_num - k >= 0 {
                Solution::dfs(deep + 1, val * 10 + latest_num - k, n, k, res)
            }
            if k != 0 && 9 >= latest_num + k && latest_num + k >= 0 {
                Solution::dfs(deep + 1, val * 10 + latest_num + k, n, k, res)
            }
        }
        if deep == n {
            // println!("{},{},{},{},", deep, val, n, k);
            res.push(val)
        }
    }

    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        Solution::dfs(0, 0, n, k, &mut res);

        res
    }
}

fn main() {
    let d = Solution::nums_same_consec_diff(5, 7);
    println!("{:?}", d);
}
