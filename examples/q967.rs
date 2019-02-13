pub struct Solution {}

impl Solution {
    fn dfs(deep: i32, val: i32, n: i32, k: i32, res: &mut Vec<i32>) {
        if deep == 0 {
            for i in 0..=9 {
                Solution::dfs(1, i, n, k, res)
            }
        } else if deep < n && val != 0 {
            let latest_num = val % 10;

            if 9 >= latest_num - k && latest_num - k >= 0 {
                Solution::dfs(deep + 1, val * 10 + latest_num - k, n, k, res)
            }
            if k != 0 {
                if 9 >= latest_num + k && latest_num + k >= 0 {
                    Solution::dfs(deep + 1, val * 10 + latest_num + k, n, k, res)
                }
            }
        }
        if deep == n {
            // println!("{},{},{},{},", deep, val, n, k);
            res.push(val)
        }
    }

    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        Solution::dfs(0, 0, n, k, &mut res);

        // fn d() {
        //     println!("{}",k)
        // }

        return res;
    }
}

fn main() {
    let d = Solution::nums_same_consec_diff(5, 7);
    println!("{:?}", d);
}
