/*

设计一种算法，打印 N 皇后在 N × N 棋盘上的各种摆法，其中每个皇后都不同行、不同列，也不在对角线上。这里的“对角线”指的是所有的对角线，不只是平分整个棋盘的那两条对角线。

注意：本题相对原题做了扩展

示例:

 输入：4
 输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 解释: 4 皇后问题存在如下两个不同的解法。
[
[".Q..",  // 解法 1
"...Q",
"Q...",
"..Q."],

["..Q.",  // 解法 2
"Q...",
"...Q",
".Q.."]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/eight-queens-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut result: Vec<Vec<String>> = vec![];
        let mut v = vec![0; n];

        fn solve(v: &mut Vec<usize>, i: usize, result: &mut Vec<Vec<String>>) {
            let n = v.len();
            if i == v.len() {
                // dbg!(&v);

                let mut mat = vec![];
                for x in v {
                    let mut res = vec![b'.'; n];
                    res[*x] = b'Q';
                    mat.push(String::from_utf8(res).unwrap());
                }
                // dbg!(&mat);

                result.push(mat);
                return;
            }

            //
            // 遍历可以填的数字
            for x in 0..v.len() {
                //假装填入数字
                v[i] = x;
                let mut checked = true;

                // 遍历已经填的数字
                for j in 0..i {
                    if v[j] == v[i]
                        || (v[j] as i32 - v[i] as i32).pow(2) == (i as i32 - j as i32).pow(2)
                    {
                        checked = false;
                        break;
                    }
                }
                if checked {
                    solve(v, i + 1, result);
                }
                // 还原
                v[i] = 0;
            }
        }
        //

        solve(&mut v, 0, &mut result);

        return result;
    }
}

fn main() {
    let res = Solution::solve_n_queens(8);
    dbg!(&res);
    dbg!(&res.len());
}

struct Solution {}
