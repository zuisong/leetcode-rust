// https://leetcode-cn.com/problems/largest-rectangle-in-histogram/
// 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
//
// 求在该柱状图中，能够勾勒出来的矩形的最大面积。

pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();

        let mut result = 0;

        for (idx, height) in heights.iter().enumerate() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] >= *height {
                let i = stack.pop().unwrap();
                result = result.max(
                    heights[i] * (idx as i32 - 1 - stack.last().map(|it| *it as i32).unwrap_or(-1)),
                );
            }
            stack.push(idx);
        }

        //        dbg!(&stack);

        while let Some(idx) = stack.pop() {
            result = result.max(
                heights[idx]
                    * (heights.len() as i32 - 1 - stack.last().map(|it| *it as i32).unwrap_or(-1)),
            );
        }

        result
    }
}

fn main() {
    let res = Solution::largest_rectangle_area(vec![5, 4, 1, 2]);
    println!("{}", res);
}
