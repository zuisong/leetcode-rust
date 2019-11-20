/*
 * @lc app=leetcode.cn id=223 lang=rust
 *
 * [223] 矩形面积
 *
 * https://leetcode-cn.com/problems/rectangle-area/description/
 *
 * algorithms
 * Medium (41.17%)
 * Total Accepted:    1.6K
 * Total Submissions: 3.8K
 * Testcase Example:  '-3\n0\n3\n4\n0\n-1\n9\n2'
 *
 * 在二维平面上计算出两个由直线构成的矩形重叠后形成的总面积。
 *
 * 每个矩形由其左下顶点和右上顶点坐标表示，如图所示。
 *
 *
 *
 * 示例:
 *
 * 输入: -3, 0, 3, 4, 0, -1, 9, 2
 * 输出: 45
 *
 * 说明: 假设矩形面积不会超出 int 的范围。
 *
 */
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let x1 = a.max(e);
        let y1 = b.max(f);
        let x2 = c.min(g);
        let y2 = d.min(h);

        let temp = if x1 >= x2 || y1 >= y2 {
            0
        } else {
            (x2 - x1) * (y2 - y1)
        };

        (c - a) * (d - b) + (g - e) * (h - f) - temp
    }
}

struct Solution {}

fn main() {

}