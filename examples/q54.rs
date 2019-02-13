// https://leetcode-cn.com/problems/spiral-matrix/
//     给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。
//
//     示例 1:
//
//     输入:
//     [
//      [ 1, 2, 3 ],
//      [ 4, 5, 6 ],
//      [ 7, 8, 9 ]
//     ]
//     输出: [1,2,3,6,9,8,7,4,5]
//     示例 2:
//
//     输入:
//     [
//       [1, 2, 3, 4],
//       [5, 6, 7, 8],
//       [9,10,11,12]
//     ]
//     输出: [1,2,3,4,8,12,11,10,9,5,6,7]

pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        if matrix.is_empty() || matrix.first().unwrap().is_empty() {
            return vec![];
        }

        let mut x_s: usize = 0;
        let mut x_e: usize = matrix.len() - 1;
        let mut y_s: usize = 0;
        let mut y_e: usize = matrix.first().unwrap().len() - 1;


        let mut direction = 1;

        while x_s <= x_e && y_s <= y_e {
            match direction {
                1 => {
                    for idx in y_s..=y_e {
                        result.push(matrix[x_s][idx])
                    }
                    direction += 1;
                    x_s += 1;
                }
                2 => {
                    for idx in x_s..=x_e {
                        result.push(matrix[idx][y_e])
                    }
                    direction += 1;
                    if y_e == 0 { break; }
                    y_e -= 1;
                }
                3 => {
                    for idx in (y_s..=y_e).rev() {
                        result.push(matrix[x_e][idx])
                    }
                    direction += 1;
                    x_e -= 1;
                }
                4 => {
                    for idx in (x_s..=x_e).rev() {
                        result.push(matrix[idx][y_s])
                    }
                    direction += 1;
                    y_s += 1;
                }
                _ => {
                    direction = 1;
                }
            }
        }
        result
    }
}

#[warn(allow_variables)]
fn main() {
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    let matrix2: Vec<Vec<i32>> = vec![
        vec![1],
        vec![5],
        vec![9]
    ];


    let res = Solution::spiral_order(matrix);
    println!("{:?}", res)
}

