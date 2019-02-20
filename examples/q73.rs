struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix.first().unwrap().is_empty() {
            return;
        }

        let x0 = matrix.first().unwrap().iter().any(|it| *it == 0);
        let y0 = matrix.iter().map(|it| it[0]).any(|it| it == 0);

        let len_x = matrix.len();
        let len_y = matrix.first().unwrap().len();

        for ix in 1..len_x {
            for iy in 1..len_y {
                if matrix[ix][iy] == 0 {
                    matrix[ix][0] = 0;
                    matrix[0][iy] = 0;
                }
            }
        }

        for ix in 1..len_x {
            for iy in 1..len_y {
                if matrix[0][iy] == 0 || matrix[ix][0] == 0 {
                    matrix[ix][iy] = 0;
                }
            }
        }

        if y0 {
            for ix in 0..len_x {
                matrix[ix][0] = 0;
            }
        }
        if x0 {
            for iy in 0..len_y {
                matrix[0][iy] = 0;
            }
        }
    }
}

fn main() {
    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![0, 0, 6], vec![7, 8, 9]];
    Solution::set_zeroes(&mut matrix);

    matrix.iter().for_each(|it| {
        println!("{:?}", it);
    });
}
