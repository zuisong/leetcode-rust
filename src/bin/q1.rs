use std::collections::HashMap;

fn main() {
    let mut rng = fastrand::Rng::new();
    let n = 10000;
    let mut arr: Vec<_> = vec![0i32; n as usize];
    let target = rng.choice(0..n).unwrap_or_default();
    arr.fill_with(|| rng.i32(i32::MIN..i32::MAX));
    //    dbg!(&arr);
    let time1 = std::time::Instant::now();
    let _res1 = solve1(&arr, target);
    let time2 = std::time::Instant::now();
    let _res2 = solve2(&arr, target);
    let time3 = std::time::Instant::now();

    println!("{:?}", time2 - time1);
    println!("{:?}", time3 - time2);

    //    dbg!(&res1);
    //    dbg!(&res2);
}

fn solve1(numbers: &Vec<i32>, target: i32) -> (usize, usize) {
    let l = numbers.len();

    for i in 0..l {
        for j in i..l {
            if numbers[i] == target - numbers[j] {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn solve2(numbers: &Vec<i32>, target: i32) -> (usize, usize) {
    let mut map = HashMap::with_capacity(10000);
    for i in 0..numbers.len() {
        map.insert(numbers[i], i);
    }
    for i in 0..numbers.len() {
        if let Some(&j) = map.get(&(target - numbers[i])) {
            return (i, j);
        }
    }
    (0, 0)
}
