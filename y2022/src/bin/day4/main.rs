use std::{cmp::max, cmp::min,fs};

fn main() {
    let mut content = fs::read_to_string("./y2022/src/bin/day4/input.txt")
        .expect("Should have been able to read the file");
    content.pop();

    let pairs: Vec<Vec<&str>> = content
        .split("\n")
        .map(|s| s.split(",").collect())
        .collect();

    let mut res = 0;
    let mut overlaps = 0;
    for pair in pairs {
        let pair0: Vec<i32> = pair[0]
            .split("-")
            .map(|i| i.parse().expect("Failed to parse number"))
            .collect();

        let pair1: Vec<i32> = pair[1]
            .split("-")
            .map(|i| i.parse().expect("Failed to parse number"))
            .collect();

        if (pair0[0] <= pair1[0] && pair1[1] <= pair0[1])
            || (pair1[0] <= pair0[0] && pair0[1] <= pair1[1])
        {
            res += 1;
        }

        let max_start = max(pair0[0], pair1[0]);
        let min_end = min(pair0[1], pair1[1]);

        if max_start <= min_end {
            overlaps += 1;
       }
    }

    println!("Answer part 1: {res}");
    println!("Answer part 2: {overlaps}");
}
