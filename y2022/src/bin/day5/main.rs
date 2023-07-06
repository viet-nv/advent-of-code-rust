use std::{fs, path::Path};

#[derive(Debug)]
struct Cmd {
    from: usize,
    to: usize,
    qty: usize,
}

#[derive(Debug)]
struct Input {
    stacks: Vec<Vec<char>>,
    cmds: Vec<Cmd>,
}
fn read_input() -> Input {
    let mut content = fs::read_to_string(Path::new("./y2022/src/bin/day5/input.txt"))
        .expect("Should have been able to read the file");
    content.pop();

    let temp: Vec<&str> = content.split("\n\n").collect();

    let raw_stack = temp[0];
    let rows: Vec<&str> = raw_stack.split("\n").collect();
    let n = rows.len() - 2;

    let m = rows
        .last()
        .unwrap_or(&"")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>()
        .len();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..m {
        stacks.push(vec![]);
    }

    for i in (0..=n).rev() {
        let mut count = 0;
        let mut number_space = 0;
        for c in rows[i].chars() {
            if c != '[' && c != ' ' && c != ']' {
                if number_space > 0 {
                    count += (number_space - 1) / 4;
                }

                if number_space == 4 {
                    count += 1;
                }

                stacks[count].push(c);
                count += 1;
                number_space = 0;
            }
            if c == ' ' {
                number_space += 1;
            }
        }
    }

    let raw_moves = temp[1];

    let cmds: Vec<Cmd> = raw_moves
        .split("\n")
        .map(|item| {
            let v: Vec<usize> = item
                .split(" ")
                .map(|i| i.parse().unwrap_or(0))
                .filter(|i| *i != 0)
                .collect();
            Cmd {
                qty: v[0],
                from: v[1] - 1,
                to: v[2] - 1,
            }
        })
        .collect();

    Input { stacks, cmds }
}

fn main() {
    let Input { mut stacks, cmds } = read_input();

    for cmd in cmds {
        for _ in 0..cmd.qty {
            let c = stacks[cmd.from].pop();
            stacks[cmd.to].push(c.unwrap());
        }
    }

    let mut r = "".to_string();
    for s in stacks {
        r.push(*s.last().unwrap());
    }

    println!("Result: {r}");
}