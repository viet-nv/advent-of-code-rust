use std::fs;

fn part1(rounds: &Vec<Vec<&str>>) -> u32 {
    let mut score = 0;
    for round in rounds {
        let shape_score = match round[1] {
            "X" => 1,
            "Y" => 2,
            _ => 3,
        };
        score += shape_score;
        let part1_game_score = match (round[0], round[1]) {
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            (_, _) => 6,
        };
        score += part1_game_score;
    }
    score
}

fn part2(rounds: &Vec<Vec<&str>>) -> u32 {
    let mut score = 0;
    for round in rounds {
        match round[1] {
            "X" => match round[0] {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => panic!("Wrong input, only accept A, B, C"),
            },
            "Y" => match round[0] {
                "A" => score += 4,
                "B" => score += 5,
                "C" => score += 6,
                _ => panic!("Wrong input, only accept A, B, C"),
            },
            "Z" => match round[0] {
                "A" => score += 8,
                "B" => score += 9,
                "C" => score += 7,
                _ => panic!("Wrong input, only accept A, B, C"),
            },
            _ => panic!("Wrong input, only accept X, Y, Z"),
        }
    }

    score
}

fn main() {
    let mut content = fs::read_to_string("./y2022/src/bin/day2/input.txt")
        .expect("Should have been able to read the file");
    content.pop(); // pop the last \n
                   //
    let rounds: Vec<Vec<&str>> = content
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect();

    // Part 1
    println!("Answer Part 1: {}", part1(&rounds));

    // Part 2
    println!("Answer Part 2: {}", part2(&rounds));
}
