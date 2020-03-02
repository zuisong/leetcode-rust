//给定一个整数 n，求以 1 ... n 为节点组成的二叉搜索树有多少种？
//
// 示例:
//
// 输入: 3
//输出: 5
//解释:
//给定 n = 3, 一共有 5 种不同结构的二叉搜索树:
//
//   1         3     3      2      1
//    \       /     /      / \      \
//     3     2     1      1   3      2
//    /     /       \                 \
//   2     1         2                 3
// Related Topics 树 动态规划

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![0; n + 1];

        dp[0] = 1;
        dp[1] = 1;

        /**
         * dp[i]  =dp[0]*dp[i-1] + dp[1]*dp[i-2] + dp[2]*dp[i-3]+ ... + dp[i-1]*dp[0]
         */
        for i in 1..=n {
            let mut res = 0;
            for j in 0..i {
                res += dp[j] * dp[i - 1 - j];
            }
            dp[i] = res;
        }

        return dp[n];
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let i = Solution::num_trees(3);
    dbg!(i);
}

struct Solution {}
