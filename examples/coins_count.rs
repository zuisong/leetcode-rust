use std::time::Instant;

/// 有面值1,5,10，20,50,100的人民币，求问10000有多少种组成方法？
fn main() {
    for i in (1000..=10000).step_by(1000) {
        println!("n={}", i);
        func1(i);
        println!();
        func2(i);
        println!("===");
    }
}

/// 动态规划解法
fn func1(target: usize) {
    let start = Instant::now();

    let coins = [5, 10, 20, 50, 100];
    let mut table = vec![1 as u64; target + 1];
    for j in 0..coins.len() {
        for i in coins[j]..=target {
            table[i] += table[i - coins[j]];
        }
    }
    let end = Instant::now();
    println!("结果 {}", table[target]);
    println!("运行时间 {:?}", end - start);
}

use std::collections::HashMap;

///
/// 递归解
fn func2(target: usize) {
    /// 用指定的几种面值的钱，组合成目标值, 有多少种可能
    ///
    /// # 参数
    /// * coins 表示硬币类型
    /// * k 表示可以使用的硬币类型  总类型的前几种
    /// * target 组合的目标
    /// * cache 缓存用 减少重复计算的
    fn count(coins: &[i32], k: usize, target: i32, cache: &mut HashMap<(usize, i32), u64>) -> u64 {
        // 目标值是0 递归结束 有一种可能性
        if target == 0 {
            return 1;
        }
        // 可以使用的钱币只剩一种了，而且 目标值不是0
        // 因为把一元面值的放在最后，所以，存在且仅存在一种方案
        if k == 1 {
            return 1;
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

    let coins = [1, 5, 10, 20, 50, 100];
    let start = Instant::now();
    let res = count(&coins, coins.len(), target as i32, &mut HashMap::new());
    let end = Instant::now();
    //    println!("{:?}", local);
    println!("程序运行时间是 --> {:?}", end - start);
    println!("共有 {} 种可能性", res);
}
