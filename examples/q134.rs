/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 * [134] 加油站
 *
 * https://leetcode-cn.com/problems/gas-station/description/
 *
 * algorithms
 * Medium (45.30%)
 * Total Accepted:    4.2K
 * Total Submissions: 9.3K
 * Testcase Example:  '[1,2,3,4,5]\n[3,4,5,1,2]'
 *
 * 在一条环路上有 N 个加油站，其中第 i 个加油站有汽油 gas[i] 升。
 *
 * 你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。
 *
 * 如果你可以绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1。
 *
 * 说明:
 *
 *
 * 如果题目有解，该答案即为唯一答案。
 * 输入数组均为非空数组，且长度相同。
 * 输入数组中的元素均为非负数。
 *
 *
 * 示例 1:
 *
 * 输入:
 * gas  = [1,2,3,4,5]
 * cost = [3,4,5,1,2]
 *
 * 输出: 3
 *
 * 解释:
 * 从 3 号加油站(索引为 3 处)出发，可获得 4 升汽油。此时油箱有 = 0 + 4 = 4 升汽油
 * 开往 4 号加油站，此时油箱有 4 - 1 + 5 = 8 升汽油
 * 开往 0 号加油站，此时油箱有 8 - 2 + 1 = 7 升汽油
 * 开往 1 号加油站，此时油箱有 7 - 3 + 2 = 6 升汽油
 * 开往 2 号加油站，此时油箱有 6 - 4 + 3 = 5 升汽油
 * 开往 3 号加油站，你需要消耗 5 升汽油，正好足够你返回到 3 号加油站。
 * 因此，3 可为起始索引。
 *
 * 示例 2:
 *
 * 输入:
 * gas  = [2,3,4]
 * cost = [3,4,3]
 *
 * 输出: -1
 *
 * 解释:
 * 你不能从 0 号或 1 号加油站出发，因为没有足够的汽油可以让你行驶到下一个加油站。
 * 我们从 2 号加油站出发，可以获得 4 升汽油。 此时油箱有 = 0 + 4 = 4 升汽油
 * 开往 0 号加油站，此时油箱有 4 - 3 + 2 = 3 升汽油
 * 开往 1 号加油站，此时油箱有 3 - 3 + 3 = 3 升汽油
 * 你无法返回 2 号加油站，因为返程需要消耗 4 升汽油，但是你的油箱只有 3 升汽油。
 * 因此，无论怎样，你都不可能绕环路行驶一周。
 *
 */
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut v = vec![0; len];
        for i in 0..len {
            v[i] = gas[i] - cost[i];
        }
        let sum: i32 = v.iter().sum();

        if sum < 0 {
            return -1;
        }
        dbg!(&v);


        if len == 1 {
            return 0;
        }

        for idx in 0..len {
            let idx2 = if idx == 0 { len - 1 } else { idx - 1 };
            if v[idx] > 0 && v[idx2] < 0 {
                let mut i = idx;
                let mut temp_sum = 0;
                let mut flag = true;
                while i != idx2 {
                    temp_sum += v[i];

                    if temp_sum <= 0 {
                        flag = false;
                        break;
                    };

                    i += 1;
                    if i == len {
                        i = 0;
                    };
                }
                if flag {
                    return idx as i32;
                }
            }
        }

        unreachable!("代码错了")
    }
}

fn main() {
//    [5,1,2,3,4]\n[4,4,1,5,1]'
    let gas = vec![5, 1, 2, 3, 4];
    let cost = vec![4, 4, 1, 5, 1];

    //    let gas = vec![2, 3, 4];
    //    let cost = vec![3, 4, 3];

    let res = Solution::can_complete_circuit(gas, cost);
    dbg!(res);
}

struct Solution {}
