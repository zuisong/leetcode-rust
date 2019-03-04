extern crate log;
extern crate simple_logger;

use log::*;
use std::process::exit;

struct Solution {}

const EMPTY_GRID: &str = "+ ";
const QUEEN_GRID: &str = "Q ";

impl Solution {
    pub fn dfs(v: &mut Vec<i32>, layer: i32, result: &mut Vec<Vec<String>>) {
        //        if result.len() > 0 {
        //            return;
        //        }
        info!("{:?}", v);

        let n = v.len() as i32;
        if layer == n {
//                        println!("{:?}", v);

            let res: Vec<String> = v
                .iter()
                .map(|it| {
                    let mut s = vec![EMPTY_GRID; n as usize];
                    s[*it as usize] = QUEEN_GRID;

                    let s = s.join("");
                    info!("{}", s);
                    s
                })
                .collect();

            result.push(res);
            exit(0);

            return;
        }

        for num in 0..n {
            let mut b = false;
            for i in 0..layer {
                if v[i as usize] == num || (v[i as usize] - num).abs() == (layer - i) {
                    b = true;
                    break;
                }
            }
            if !b {
                v[layer as usize] = num;
                Solution::dfs(v, layer + 1, result);
                v[layer as usize] = -1;
            }
        }
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut v = vec![-1; n as usize];
        let mut res: Vec<Vec<String>> = vec![];
        Solution::dfs(&mut v, 0, &mut res);
        return res;
    }
}

fn main() {
    simple_logger::init().unwrap();
    let result = Solution::solve_n_queens(31);

    info!("{}", result.len());

    for (idx, res) in result.iter().enumerate() {
        info!("===== {}", idx + 1);

        for r in res {
            info!("{}", r);
        }
    }
}
