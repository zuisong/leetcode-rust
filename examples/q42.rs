struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let (max_idx, _) = height.iter().enumerate()
            .max_by(|it1, it2| it1.1.cmp(it2.1)).unwrap();

        let mut result = 0;

        let mut temp = *height.first().unwrap();
        for i in 1..max_idx {
            if temp > height[i] {
                result += (temp - height[i]);
            } else {
                temp = height[i];
            }
        }

        temp = *height.last().unwrap();
        for i in (max_idx..height.len()).rev() {
            if temp > height[i] {
                result += (temp - height[i]);
            } else {
                temp = height[i];
            }
        }

        return result;
    }
}

fn main() {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let res = Solution::trap(heights);

    println!("{}", res);
}

