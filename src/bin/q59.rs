//
//      https://leetcode-cn.com/problems/spiral-matrix-ii/
//      给定一个正整数 n，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的正方形矩阵。
//
//      示例:
//
//      输入: 3
//      输出:
//
//      [
//       [ 1, 2, 3 ],
//       [ 8, 9, 4 ],
//       [ 7, 6, 5 ]
//      ]
//

pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let arr: Vec<i32> = vec![0; (n * n) as usize]; //  (1..=(n * n)).collect();
        let mut matrix: Vec<Vec<i32>> = arr.chunks(n as usize).map(Vec::from).collect();

        let mut x_s: usize = 0;
        let mut x_e: usize = matrix.len() - 1;
        let mut y_s: usize = 0;
        let mut y_e: usize = matrix.first().unwrap().len() - 1;

        let mut num = 1;

        let mut direction = 1;

        while x_s <= x_e && y_s <= y_e {
            match direction {
                1 => {
                    for idx in y_s..=y_e {
                        matrix[x_s][idx] = num;
                        num += 1;
                    }
                    x_s += 1;
                }
                2 => {
                    for idx in x_s..=x_e {
                        matrix[idx][y_e] = num;
                        num += 1;
                    }
                    if y_e == 0 {
                        break;
                    } else {
                        y_e -= 1;
                    };
                }
                3 => {
                    for idx in (y_s..=y_e).rev() {
                        matrix[x_e][idx] = num;
                        num += 1;
                    }
                    x_e -= 1;
                }
                4 => {
                    for idx in (x_s..=x_e).rev() {
                        matrix[idx][y_s] = num;
                        num += 1;
                    }
                    y_s += 1;
                }
                _ => panic!("不可能的啦"),
            };

            if direction < 4 {
                direction += 1;
            } else {
                direction = 1;
            };
        }

        matrix
    }
}

fn main() {
    let res = Solution::generate_matrix(10);

    //    dbg!(&res);

    res.iter().for_each(|r| {
        r.iter().for_each(|x| print!("{: >4}", x));
        println!();
    });
}
