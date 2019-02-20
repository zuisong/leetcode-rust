// https://leetcode-cn.com/problems/maximal-rectangle/
// 解法 https://segmentfault.com/a/1190000003498304
pub struct Solution {}

mod q84;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let y_len = matrix.first().unwrap_or(&vec![]).len();

        matrix.iter().enumerate().for_each(|(idx, arr)| {
            let mut heights: Vec<i32> = vec![0; y_len];
            arr.iter().enumerate().for_each(|(idy, _)| {
                for i in (0..=idx).rev() {
                    if matrix[i][idy] == '0' {
                        break;
                    } else {
                        heights[idy] += 1
                    }
                }
            });
            result = q84::Solution::largest_rectangle_area(heights).max(result);
        });

        return result;
    }
}

fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];

    let res = Solution::maximal_rectangle(matrix);

    println!("{}", res);
}
