/**

堆箱子。给你一堆n个箱子，箱子宽 wi、高hi、深di。箱子不能翻转，将箱子堆起来时，下面箱子的宽度、高度和深度必须大于上面的箱子。实现一种方法，搭出最高的一堆箱子。箱堆的高度为每个箱子高度的总和。

输入使用数组[wi, di, hi]表示每个箱子。

示例1:

 输入：box = [[1, 1, 1], [2, 2, 2], [3, 3, 3]]
 输出：6
示例2:

 输入：box = [[1, 1, 1], [2, 3, 4], [2, 6, 7], [3, 4, 5]]
 输出：10
提示:

箱子的数目不大于3000个。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/pile-box-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

impl Solution {
    pub fn pile_box(mut boxs: Vec<Vec<i32>>) -> i32 {
        if boxs.is_empty() {
            return 0;
        }

        boxs.sort_by_key(|c| c[2]);
        // dbg!(&boxs);
        let mut dp = vec![0; boxs.len()];
        dp[0] = boxs[0][2];
        for i in 1..boxs.len() {
            let mut max = boxs[i][2];
            for j in 0..i {
                if true
                    && boxs[i][0] > boxs[j][0]
                    && boxs[i][1] > boxs[j][1]
                    && boxs[i][2] > boxs[j][2]
                    && boxs[i][2] + dp[j] > max
                {
                    max = boxs[i][2] + dp[j];
                }
            }
            dp[i] = max;
        }

        return dp.into_iter().max().unwrap();
    }
}

fn main() {
    let boxs = vec![vec![1, 1, 1], vec![2, 3, 4], vec![2, 6, 7], vec![3, 4, 5]];
    let res = Solution::pile_box(boxs);
    println!("{:?}", res);
}

struct Solution {}
