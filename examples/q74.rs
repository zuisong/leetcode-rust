/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 *
 * https://leetcode-cn.com/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (33.46%)
 * Total Accepted:    7K
 * Total Submissions: 20.8K
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,50]]\n3'
 *
 * 编写一个高效的算法来判断 m x n 矩阵中，是否存在一个目标值。该矩阵具有如下特性：
 *
 *
 * 每行中的整数从左到右按升序排列。
 * 每行的第一个整数大于前一行的最后一个整数。
 *
 *
 * 示例 1:
 *
 * 输入:
 * matrix = [
 * ⁠ [1,   3,  5,  7],
 * ⁠ [10, 11, 16, 20],
 * ⁠ [23, 30, 34, 50]
 * ]
 * target = 3
 * 输出: true
 *
 *
 * 示例 2:
 *
 * 输入:
 * matrix = [
 * ⁠ [1,   3,  5,  7],
 * ⁠ [10, 11, 16, 20],
 * ⁠ [23, 30, 34, 50]
 * ]
 * target = 13
 * 输出: false
 *
 */
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty()||matrix[0].is_empty() {
            return false;
        }
        let len_x = matrix.len();
        let len_y = matrix.first().map(|v| v.len()).unwrap();

        let mut x1 = 0;
        let mut x2 = len_x - 1;

        if target < matrix[x1][0] {
            return false;
        }

        while x1 <= x2 {
            let mid = (x1 + x2) / 2;
            if matrix[mid][0] > target {
                if mid == 0 {
                    return false;
                }
                x2 = mid - 1;
            } else if matrix[mid][0] < target {
                x1 = mid + 1;
            } else {
                return true;
            }
        }
        let mut y1 = 0;
        let mut y2 = len_y - 1;
        while y1 <= y2 {
            let mid = (y1 + y2) / 2;
            if matrix[x2][mid] > target {
                if mid == 0 {
                    return false;
                }
                y2 = mid - 1;
            } else if matrix[x2][mid] < target {
                y1 = mid + 1;
            } else {
                return true;
            }
        }
        false
    }
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];

    let b = Solution::search_matrix(matrix, 1);

    assert_eq!(false, b);
}

struct Solution {}
