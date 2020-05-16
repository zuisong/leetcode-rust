use std::collections::{HashMap, VecDeque};
use std::process::exit;

use log::info;

fn main() {
    simple_logger::init();

    let res = vec![
        vec![1, 2, 4, 15],
        vec![6, 7, 3, 14],
        vec![5, 11, 8, 13],
        vec![10, 9, 12, 0],
    ];

    let target = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 0],
    ];

    let mut pos0 = (0, 0);
    for (x, v) in res.iter().enumerate() {
        for (y, val) in v.iter().enumerate() {
            if *val == 0 {
                pos0 = (x as i32, y as i32);
            }
        }
    }
    dbg!(&pos0);

    solve(res, hash(&target), pos0);
}

fn solve(matrix: Vec<Vec<i32>>, target_hash: String, pos0: (i32, i32)) {
    let len_y = matrix.first().unwrap().len() as i32;
    let len_x = matrix.len() as i32;
    // map.remove(&hash_temp);
    let mut list = VecDeque::new();
    let mut map = HashMap::new();
    let h = hash(&matrix);
    map.insert(h, (String::from(" "), (0, 0)));
    list.push_back((matrix, pos0));

    while !list.is_empty() {
        let (re, pos0) = list.pop_front().unwrap();

        let h = hash(&re);

        if h != target_hash {
            for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)].to_vec().into_iter() {
                let mut re = re.clone();

                if pos0.0 + i < 0 || pos0.0 + i >= len_x {
                    continue;
                }

                if pos0.1 + j < 0 || pos0.1 + j >= len_y {
                    continue;
                }

                let new_pos0x: usize = (pos0.0 + i) as usize;
                let new_pos0y: usize = (pos0.1 + j) as usize;

                let temp = re[new_pos0x][new_pos0y];
                re[new_pos0x][new_pos0y] = 0;
                re[pos0.0 as usize][pos0.1 as usize] = temp;
                let hash_temp = hash(&re);

                if !map.contains_key(&hash_temp) {
                    map.insert(hash_temp, (h.clone(), (i, j)));
                    list.push_back((re, (new_pos0x as i32, new_pos0y as i32)))
                }
            }
        } else {
            let mut h = h;
            let mut route = vec![];
            while map.contains_key(&h) {
                dbg!(&h);

                let temp = map.get(&h).unwrap_or(&(String::from(""), (0, 0))).1;
                println!("{:?}", temp);
                let step = match temp {
                    (0, 1) => "右",
                    (1, 0) => "下",
                    (0, -1) => "左",
                    (-1, 0) => "上",
                    _ => "",
                };

                route.push(step);
                h = map.get(&h).unwrap().0.clone();
                // println!("{:?}", h);
            }
            route.iter().rev().for_each(|s| {
                dbg!(s);
            });

            exit(-1)
        }
    }
}

fn hash(v: &Vec<Vec<i32>>) -> String {
    serde_json::to_string(v).unwrap()
}
