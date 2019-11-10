use std::collections::HashMap;
use std::process::exit;

fn main() {
    let mut res = vec![
        vec![1, 2, 3, 4],
        vec![4, 5, 6, 7],
        vec![8, 9, 0, 11],
        vec![12, 13, 10, 14],
    ];
    let target = vec![
        vec![1, 2, 3, 4],
        vec![4, 5, 6, 7],
        vec![8, 9, 10, 11],
        vec![12, 13, 14, 0],
    ];
    let mut map = HashMap::new();
    solve(&mut res, &hash(&target), (2, 2), &mut map);
    dbg!(map);
    dbg!(res);
}

fn solve(
    re: &mut Vec<Vec<i32>>,
    target_hash: &String,
    pos0: (i32, i32),
    map: &mut HashMap<String, (String, (i32, i32))>,
) {
    let len_y = re.first().unwrap().len() as i32;
    let len_x = re.len() as i32;
    let h = hash(re);

    if h == *target_hash {
        let mut h = h;
        let mut route = vec![];
        while map.contains_key(&h) {
            let temp = map.get(&h).unwrap_or(&(String::new(), (0, 0))).1;
            //            println!("{:?}", temp);
            let step = match temp {
                (0, 1) => "右",
                (1, 0) => "下",
                (0, -1) => "左",
                (-1, 0) => "上",
                _ => unreachable!(),
            };

            route.push(step);
            h = map.get(&h).unwrap().0.to_string();
            //            println!("{:?}", h);
        }
        route.iter().rev().for_each(|s| {
            dbg!(s);
        });

        exit(0)
    } else {
        for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)].to_vec().into_iter() {
            if pos0.0 + i < 0 || pos0.0 + i >= len_x {
                continue;
            }
            if pos0.1 * j < 0 || pos0.1 + j >= len_y {
                continue;
            }

            let new_pos0x: usize = (pos0.0 + i) as usize;
            let new_pos0y: usize = (pos0.1 + j) as usize;

            let temp = re[new_pos0x][new_pos0y];
            re[new_pos0x][new_pos0y] = 0;
            re[pos0.0 as usize][pos0.1 as usize] = temp;
            let hash_temp = hash(&re);

            if !map.contains_key(&hash_temp) {
                map.insert(hash_temp.clone(), (h.clone(), (i, j)));
                solve(re, target_hash, (new_pos0x as i32, new_pos0y as i32), map);
                map.remove(&hash_temp);
                re[new_pos0x][new_pos0y] = temp;
                re[pos0.0 as usize][pos0.1 as usize] = 0;
            }
        }
    }
}

fn hash(v: &Vec<Vec<i32>>) -> String {
    format!("{:?}", v)
}
