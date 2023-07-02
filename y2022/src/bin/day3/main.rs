use std::{fs, path::Path};

fn get_idx(c: &char) -> usize {
    if c.is_lowercase() {
        *c as usize - 'a' as usize
    } else {
        *c as usize - 'A' as usize + 26
    }
}

fn get_common(arr: Vec<Vec<char>>) -> u32 {
    let mut check = [false; 53];

    for (i, v) in arr.iter().enumerate() {
        if i == 0 {
            for &i in v {
                check[get_idx(&i)] = true;
            }
        } else {
            let mut temp = [false; 53];
            for &i in v {
                temp[get_idx(&i)] = true;
            }
            for i in 0..53 {
                check[i] = check[i] && temp[i]
            }
        }
    }

    let mut res: u32 = 0;
    for i in 0..53 {
        if check[i] {
            res = i as u32 + 1;
        }
    }
    res
}

fn main() {
    let mut content = fs::read_to_string(Path::new("./y2022/src/bin/day3/input.txt"))
        .expect("Should have been able to read the file");
    content.pop();
    let rucksacks: Vec<&str> = content.split("\n").collect();

    let mut res = 0;
    for rucksack in &rucksacks {
        let chars: Vec<char> = rucksack.chars().collect();
        let mid = chars.len() / 2;
        res += get_common(vec![chars[0..mid].to_vec(), chars[mid..].to_vec()]);
    }

    println!("Answer Part 1: {res}");

    let n = rucksacks.len();
    let mut res = 0;
    for i in (0..n).step_by(3) {
        res += get_common(vec![
            rucksacks[i].chars().collect(),
            rucksacks[i + 1].chars().collect(),
            rucksacks[i + 2].chars().collect(),
        ])
    }

    println!("Answer Part 2: {res}");
}
