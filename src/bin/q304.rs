//给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2)。
//
//
//上图子矩阵左上角 (row1, col1) = (2, 1) ，右下角(row2, col2) = (4, 3)，该子矩形内元素的总和为 8。
//
// 示例:
//
// 给定 matrix = [
//  [3, 0, 1, 4, 2],
//  [5, 6, 3, 2, 1],
//  [1, 2, 0, 1, 5],
//  [4, 1, 0, 1, 7],
//  [1, 0, 3, 0, 5]
//]
//
//sumRegion(2, 1, 4, 3) -> 8
//sumRegion(1, 1, 2, 2) -> 11
//sumRegion(1, 2, 2, 4) -> 12
//
//
// 说明:
//
//
// 你可以假设矩阵不可变。
// 会多次调用 sumRegion 方法。
// 你可以假设 row1 ≤ row2 且 col1 ≤ col2。
//
// Related Topics 动态规划

//leetcode submit region begin(Prohibit modification and deletion)
struct NumMatrix {
    sum_matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        let row_len = matrix.len();
        let col_len = matrix[0].len();

        for row in 1..row_len {
            for col in 1..col_len {}
        }

        return NumMatrix { sum_matrix: matrix };
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        return 0;
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let matrix = [
        [3, 0, 1, 4, 2],
        [5, 6, 3, 2, 1],
        [1, 2, 0, 1, 5],
        [4, 1, 0, 1, 7],
        [1, 0, 3, 0, 5],
    ]
    .iter()
    .map(|it| it.to_vec())
    .collect::<Vec<_>>()
    .to_vec();

    let num_matrix = NumMatrix::new(matrix);

    let result = num_matrix.sum_region(2, 1, 3, 2);
    println!("{:?}", result);
}
