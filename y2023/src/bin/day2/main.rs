use std::fs;

struct ColorLimit {
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
    let color_limit = ColorLimit {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = read_input();

    let part1: i32 = games
        .iter()
        .filter(|game| {
            game.cubes.iter().all(|cube| {
                cube.red <= color_limit.red
                    && cube.blue <= color_limit.blue
                    && cube.green <= color_limit.green
            })
        })
        .map(|game| game.id)
        .sum();
    println!("Part One: {}", part1);

    let part2: i32 = games
        .iter()
        .map(|game| {
            let max_red = game.cubes.iter().map(|cube| cube.red).max().unwrap_or(0);
            let max_green = game.cubes.iter().map(|cube| cube.green).max().unwrap_or(0);
            let max_blue = game.cubes.iter().map(|cube| cube.blue).max().unwrap_or(0);
            max_red * max_green * max_blue
        })
        .sum();
    println!("Part Two: {}", part2);
}
