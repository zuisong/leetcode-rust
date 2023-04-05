//我们有一系列公交路线。每一条路线 routes[i] 上都有一辆公交车在上面循环行驶。例如，有一条路线 routes[0] = [1, 5, 7]，表示第一
//辆 (下标为0) 公交车会一直按照 1->5->7->1->5->7->1->... 的车站路线行驶。
//
// 假设我们从 S 车站开始（初始时不在公交车上），要去往 T 站。 期间仅可乘坐公交车，求出最少乘坐的公交车数量。返回 -1 表示不可能到达终点车站。
//
//
//示例:
//输入:
//routes = [[1, 2, 7], [3, 6, 7]]
//S = 1
//T = 6
//输出: 2
//解释:
//最优策略是先乘坐第一辆公交车到达车站 7, 然后换乘第二辆公交车到车站 6。
//
//
// 说明:
//
//
// 1 <= routes.length <= 500.
// 1 <= routes[i].length <= 500.
// 0 <= routes[i][j] < 10 ^ 6.
//
// Related Topics 广度优先搜索

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
        use std::collections::*;
        let mut map = HashMap::new();

        for (route_idx, route) in routes.iter().enumerate() {
            for x in route {
                map.entry(x).or_insert(HashSet::new()).insert(route_idx);
            }
        }

        let mut to_visited = VecDeque::new();

        // (当前换乘了几次 当前所在站点 已经搭乘过的车次   )
        to_visited.push_back((0, s, HashSet::new()));

        while let Some(station) = to_visited.pop_front() {
            if station.1 == t {
                return station.0;
            }

            map.get(&station.1)
                .unwrap()
                .iter()
                .filter(|it| !station.2.contains(it))
                .for_each(|idx| {
                    routes.get(*idx).unwrap().iter().for_each(|it| {
                        let mut vec = station.2.clone();
                        vec.insert(idx);
                        to_visited.push_back((station.0 + 1, *it, vec));
                    })
                })
        }

        -1
    }
}

//leetcode submit region end(Prohibit modification and deletion)
struct Solution {}

fn main() {
    use std::time::Instant;
    let v = vec![vec![1, 2, 7], vec![3, 6, 7], vec![5, 6, 11]];

    let now = Instant::now();
    let s = Solution::num_buses_to_destination(v, 1, 11);
    let now2 = Instant::now();

    dbg!(s);
    dbg!(now2 - now);
}
