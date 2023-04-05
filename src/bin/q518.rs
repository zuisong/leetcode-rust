/*

给定不同面额的硬币和一个总金额。写出函数来计算可以凑成总金额的硬币组合数。假设每一种面额的硬币有无限个。



示例 1:

输入: amount = 5, coins = [1, 2, 5]
输出: 4
解释: 有四种方式可以凑成总金额:
5=5
5=2+2+1
5=2+1+1+1
5=1+1+1+1+1
示例 2:

输入: amount = 3, coins = [2]
输出: 0
解释: 只用面额2的硬币不能凑成总金额3。
示例 3:

输入: amount = 10, coins = [10]
输出: 1


注意:

你可以假设：

0 <= amount (总金额) <= 5000
1 <= coin (硬币面额) <= 5000
硬币种类不超过 500 种
结果符合 32 位符号整数

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/coin-change-2
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; (amount + 1) as usize];
        dp[0] = 1;
        for (_, &n) in coins.iter().enumerate() {
            for i in (n as usize)..dp.len() {
                dp[i] += dp[i - n as usize];
            }
        }

        *dp.last().unwrap()
    }

    ///
    /// 递归解
    pub fn change2(amount: i32, coins: Vec<i32>) -> i32 {
        /// 用指定的几种面值的钱，组合成目标值, 有多少种可能
        ///
        /// # 参数
        /// * coins 表示硬币类型
        /// * k 表示可以使用的硬币类型  总类型的前几种
        /// * target 组合的目标
        /// * cache 缓存用 减少重复计算的
        fn count(
            coins: &Vec<i32>,
            k: usize,
            target: i32,
            cache: &mut HashMap<(usize, i32), i32>,
        ) -> i32 {
            // 目标值是0 递归结束 有一种可能性
            if target == 0 {
                return 1;
            }
            if k == 0 {
                return 0;
            }
            // 读缓存，缓存有的话就不要计算了
            if cache.contains_key(&(k, target)) {
                return *cache.get(&(k, target)).unwrap();
            }
            let mut res = 0;
            // 从用 0张 到用N张 第 k 种面值的钱 用完则不再用这种面值的钱
            // 问题转化成用前K-1种面值的钱组合成 target-n*coins[k - 1] 的可能性的和
            // 递归走起
            for dealed in (0..=target).step_by(coins[k - 1] as usize) {
                res += count(coins, k - 1, target - dealed, cache);
            }
            // 记录进缓存信息里
            cache.insert((k, target), res);
            res
        }

        count(&coins, coins.len(), amount, &mut HashMap::new())
    }
}

struct Solution {}

fn main() {
    for i in (100..=1000).step_by(100) {
        println!("n={}", i);

        println!("{}", Solution::change(i, vec![1, 5, 10, 20, 50, 100]));

        println!("{}", Solution::change2(i, vec![1, 5, 10, 20, 50, 100]));
    }
}
