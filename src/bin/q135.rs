//老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。
//
// 你需要按照以下要求，帮助老师给这些孩子分发糖果：
//
//
// 每个孩子至少分配到 1 个糖果。
// 相邻的孩子中，评分高的孩子必须获得更多的糖果。
//
//
// 那么这样下来，老师至少需要准备多少颗糖果呢？
//
// 示例 1:
//
// 输入: [1,0,2]
//输出: 5
//解释: 你可以分别给这三个孩子分发 2、1、2 颗糖果。
//
//
// 示例 2:
//
// 输入: [1,2,2]
//输出: 4
//解释: 你可以分别给这三个孩子分发 1、2、1 颗糖果。
//     第三个孩子只得到 1 颗糖果，这已满足上述两个条件。
// Related Topics 贪心算法

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];
        for i in 1..n {
            //从前往后遍历ratings数组
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..(n - 2)).rev() {
            //从后往前遍历ratings数组
            if candies[i] <= candies[i + 1] && ratings[i] > ratings[i + 1] {
                candies[i] = candies[i + 1] + 1;
            }
        }
        println!("{:?}", &ratings);
        println!("{:?}", &candies);
        let sum = candies.iter().sum();
        dbg!(sum);

        println!();
        return sum;
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    assert_eq!(4, Solution::candy(vec![1, 2, 2]));
    assert_eq!(5, Solution::candy(vec![1, 0, 2]));
    Solution::candy(vec![1, 0, 2, 2, 3]);
}

struct Solution {}
