use std::fs;

struct Config {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Cube {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    cubes: Vec<Cube>,
}

fn read_input() -> Vec<Game> {
    let mut content = fs::read_to_string("./y2023/src/bin/day2/input.txt")
        .expect("show have been able to read the file");

    // Remove the last newline character if it exists
    if content.ends_with('\n') {
        content.pop();
    }
    let mut result: Vec<Game> = Vec::new();
    content.lines().for_each(|s| {
        // Game 1: 12 red, 2 green, 5 blue; 9 red, 6 green, 4 blue; 10 red, 2 green, 5 blue; 8 blue, 9 red
        let mut parts = s.split(":");
        let id = parts
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut cubes: Vec<Cube> = Vec::new();
        parts.next().unwrap().split(";").for_each(|c| {
            // 12 red, 2 green, 5 blue
            let cube_parts = c.split(",");
            let mut cube = Cube {
                red: 0,
                blue: 0,
                green: 0,
            };

            for part in cube_parts {
                let mut colors = part.trim().split_whitespace();
                let count = colors.next().unwrap().parse::<i32>().unwrap();
                let color = colors.next().unwrap();
                match color {
                    "red" => cube.red = count,
                    "blue" => cube.blue = count,
                    "green" => cube.green = count,
                    _ => {}
                }
            }

            cubes.push(cube);
        });

        result.push(Game { id, cubes });
    });

    result
}

fn main() {
    let config = Config {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = read_input();

    let mut part1 = 0;
    for game in &games {
        let mut eligible = true;
        for cube in &game.cubes {
            if cube.red > config.red || cube.blue > config.blue || cube.green > config.green {
                eligible = false;
                break;
            }
        }
        if eligible {
            part1 += game.id;
        }
    }
    println!("Part One: {}", part1);

    let mut part2 = 0;
    for game in &games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for cube in &game.cubes {
            if cube.red > max_red {
                max_red = cube.red;
            }
            if cube.green > max_green {
                max_green = cube.green;
            }
            if cube.blue > max_blue {
                max_blue = cube.blue;
            }
        }
        part2 += max_red * max_green * max_blue;
    }

    println!("Part Two: {}", part2);
}
