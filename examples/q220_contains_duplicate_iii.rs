pub struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut i: i32 = 0;
        loop {
            let i1 = i;
            let i2 = i1 + k;

            if i2 as usize >= nums.len() {
                break;
            }
            if (nums[i1 as usize] - nums[i2 as usize]).abs() == t {
                return true;
            }
            i += 1;
        }
        false
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let t = 0;

    let d = Solution::contains_nearby_almost_duplicate(nums, k, t);
    println!("{}", d);
}
