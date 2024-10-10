use std::fs;

fn get_input() -> Vec<String> {
    let mut content = fs::read_to_string("./y2023/src/bin/day1/input.txt")
        .expect("Should have been able to read the file");
    // Remove the last newline character if it exists
    if content.ends_with('\n') {
        content.pop();
    }
    content.lines().map(|s| s.to_string()).collect()
}

fn calibration(s: &str) -> u32 {
    let mut first = '0';
    let mut last = '0';
    for c in s.chars() {
        if c.is_digit(10) {
            first = c;
            break;
        }
    }
    for c in s.chars().rev() {
        if c.is_digit(10) {
            last = c;
            break;
        }
    }

    format!("{first}{last}").parse::<u32>().unwrap()
}

fn first_index(s: &str, numbers: Vec<&str>) -> usize {
    let l = s.len();

    let mut first = 0;
    let mut best_idx = l;
    for (index, n) in numbers.iter().enumerate() {
        match s.find(n) {
            Some(i) => {
                if i < best_idx {
                    best_idx = i;
                    first = index + 1;
                }
            }
            None => {}
        }
    }

    for (index, c) in s.chars().enumerate() {
        if c.is_digit(10) && index < best_idx {
            first = c.to_string().parse::<usize>().unwrap();
            break;
        }
    }

    first
}

fn calibration_v2(s: &str) -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let b = vec![
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    let first = first_index(s, numbers);
    let reverse_s: String = s.to_string().chars().rev().collect();
    let last = first_index(&reverse_s, b);

    format!("{first}{last}").parse::<u32>().unwrap()
}

fn main() {
    let input = get_input();

    let mut sum = 0;
    let mut sum2 = 0;
    for s in input {
        sum += calibration(&s);
        // let a = calibration_v2(&s);
        // println!("{a}");
        sum2 += calibration_v2(&s);
    }
    println!("v1: {sum}\nv2: {sum2}");
}
