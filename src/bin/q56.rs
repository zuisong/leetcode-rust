//
//      给出一个区间的集合，请合并所有重叠的区间。
//
//      示例 1:
//
//      输入: [[1,3],[2,6],[8,10],[15,18]]
//      输出: [[1,6],[8,10],[15,18]]
//      解释: 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
//      示例 2:
//
//      输入: [[1,4],[4,5]]
//      输出: [[1,5]]
//      解释: 区间 [1,4] 和 [4,5] 可被视为重叠区间。

// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        dbg!(&intervals);

        if intervals.is_empty() {
            return vec![];
        };

        let mut intervals: Vec<Interval> = intervals
            .iter()
            .map(|it| Interval::new(it.start, it.end))
            .collect();

        intervals.sort_by_key(|it| it.start);
        let mut result: Vec<Interval> = Vec::new();
        let s = intervals.first().unwrap();
        let mut interval1 = Interval::new(s.start, s.end);

        for idx in 1..intervals.len() {
            let interval2 = &intervals[idx];
            if interval1.end >= interval2.start {
                interval1 = Interval::new(interval1.start, interval1.end.max(interval2.end));
            } else {
                result.insert(0, Interval::new(interval1.start, interval1.end));
                interval1 = Interval::new(interval2.start, interval2.end);
            };
        }
        result.insert(0, interval1);
        result
    }
}

fn main() {
    let intervals: Vec<Interval> = [[2, 3], [4, 5], [6, 7], [8, 9], [1, 4]]
        .iter()
        .map(|it| Interval::new(it[0], it[1]))
        .collect();

    //    let intervals = vec![
    //        Interval::new(1, 2),
    //        Interval::new(4, 5),
    //        Interval::new(2, 3),
    //    ];

    let res = Solution::merge(intervals);

    println!("{:?}", res);
}
