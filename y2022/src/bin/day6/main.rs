use std::fs;

fn start_of_msg(input: &str, distinct: usize) {
    let n = input.len();

    let arr: Vec<char> = input.chars().collect();

    for i in 0..(n - distinct) {
        let mut tmp: Vec<char> = arr[i..(i + distinct)].iter().copied().collect();
        tmp.sort();
        let mut ok = true;
        for j in 0..(distinct - 1) {
            if tmp[j] == tmp[j + 1] {
                ok = false;
            }
        }
        if ok {
            println!("Answer: {}", i + distinct);
            break;
        }
    }
}

fn main() {
    let content = fs::read_to_string("./y2022/src/bin/day6/input.txt")
        .expect("Should have been able to read the file");

    start_of_msg(&content, 4);
    start_of_msg(&content, 14);
}
