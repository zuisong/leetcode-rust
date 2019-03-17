use chrono::Local;
/// 有面值1,5,10，20,50,100的人民币，求问10000有多少种组成方法？
fn main() {
    let start = Local::now();

    let target: usize = 10000;
    let coins = [5, 10, 20, 50, 100];
    let mut table = vec![1 as u64; target + 1];
    for j in 0..coins.len() {
        for i in coins[j]..=target {
            table[i] += table[i - coins[j]];
        }
    }
    let end = Local::now();
    println!("结果 {}", table[target]);
    println!("运行时间 {}", end - start);
}