use std::fs;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Folder {
    name: String,
    size: usize,
    folders: Vec<Folder>,
    files: Vec<File>,
}

fn main() {
    let mut content = fs::read_to_string("./y2022/src/bin/day7/input.txt")
        .expect("Should have been able to read the file");
    content.pop();

    let mut arrays: Vec<Folder> = vec![];

    content.split("\n").for_each(|cmd| {
        if cmd.starts_with("$ cd") {
            let folder_name = cmd[5..].to_string();
            if folder_name == ".." {
                arrays.pop();
            } else {
                let new_folder = Folder {
                    name: folder_name,
                    size: 0,
                    folders: vec![],
                    files: vec![],
                };
                arrays.push(new_folder)
            }
        } else if cmd == "$ ls" {
            //
        } else if cmd.starts_with("dir ") {
            let new_folder = Folder {
                name: cmd[4..].to_string(),
                size: 0,
                folders: vec![],
                files: vec![],
            };
            arrays.last_mut().unwrap().folders.push(new_folder);
        } else {
            let tmp: Vec<&str> = cmd.split(" ").collect();
            let size: usize = tmp[0].parse().unwrap_or(0);
            let file_name: &str = tmp[1];

            arrays.last_mut().unwrap().files.push(File {
                name: file_name.to_owned(),
                size,
            });
        }
    });

    println!("Answer Part 1: {:#?}", arrays);
}
