fn main() {
    let str1 = "gfgfdsgdge".to_string();
    let str2 = "owfewfwefgdge".to_string();
    let s = common_subseq(&str1, &str2);
    println!("{:?}", s);
    let s = common_substr(&str1, &str2);
    println!("{:?}", s);
}

fn common_substr(str1: &String, str2: &String) -> String {
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();

    let mut v = vec![vec![0; str2.len() + 1]; str1.len() + 1];

    let mut p = (0, 0);
    for i in 0..s1.len() {
        for j in 0..s2.len() {
            v[i + 1][j + 1] = if s1[i] == s2[j] { v[i][j] + 1 } else { 0 };
            if v[i + 1][j + 1] > v[p.0][p.1] {
                p = (i + 1, j + 1);
            }
        }
    }

    v.iter().for_each(|it| println!("{:?}", it));
    dbg!(&p);
    str1.chars()
        .skip(p.0 - v[p.0][p.1])
        .take(v[p.0][p.1])
        .collect()
}

fn common_subseq(str1: &String, str2: &String) -> usize {
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();

    let mut v = vec![vec![0; str2.len() + 1]; str1.len() + 1];

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            v[i + 1][j + 1] =
                if s1[i] == s2[j] {
                    v[i][j] + 1
                } else {
                    v[i + 1][j].max(v[i][j + 1])
                };
        }
    }

    v.iter().for_each(|it| println!("{:?}", it));
    *v.last().unwrap().last().unwrap() as usize
}
