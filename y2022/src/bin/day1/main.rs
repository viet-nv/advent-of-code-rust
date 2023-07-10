use std::fs;

fn calories_counting(calories: &Vec<Vec<i32>>) -> i32 {
    let mut res: i32 = -1;
    for c in calories.iter() {
        let sum = c.iter().sum();
        if res < sum {
            res = sum;
        }
    }
    res
}

fn calories_counting_2(calories: &Vec<Vec<i32>>) -> i32 {
    let mut res: i32 = -1;
    let mut r: Vec<i32> = calories.iter().map(|x| x.iter().sum()).collect();
    r.sort();
    r.reverse();

    for c in calories.iter() {
        let sum = c.iter().sum();
        if res < sum {
            res = sum;
        }
    }
    r[0] + r[1] + r[2]
}

fn get_input() -> Vec<Vec<i32>> {
    let mut content = fs::read_to_string("./y2022/src/bin/day1/input.txt")
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
    let input = get_input();
    println!("Answer Part 1: {}", calories_counting(&input));
    println!("Answer Part 2: {}", calories_counting_2(&input));
}
