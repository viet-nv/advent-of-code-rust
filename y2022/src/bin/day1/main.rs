use std::{fs, path::Path};

fn calories_counting(calories: Vec<Vec<i32>>) -> i32 {
    let mut res: i32 = -1;
    for c in calories.iter() {
        let sum = c.iter().sum();
        if res < sum {
            res = sum;
        }
    }
    res
}

fn get_input() -> Vec<Vec<i32>> {
    let mut content = fs::read_to_string(Path::new("./y2022/src/bin/day1/input.txt"))
        .expect("Should have been able to read the file");

    content.pop();
    content
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|i| i.parse().expect("Failed to parse number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn main() {
    let res = calories_counting(get_input());
    println!("Answer: {res}");
}
