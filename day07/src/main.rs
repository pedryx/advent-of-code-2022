use itertools::Itertools;

use std::collections::HashMap;

struct DirNode<'a> {
    size: usize,
    parent: Option<usize>,
    dirs: HashMap<&'a str, usize>,
}

fn parse_input<'a>(input: &'a str) -> Vec<DirNode> {
    let mut input = input.lines();
    input.next();

    let mut directories: Vec<DirNode> = Vec::new();
    directories.push(DirNode {
        size: 0,
        parent: None,
        dirs: HashMap::new(),
    });
    let mut current_index = 0;

    for line in input {
        let mut tokens = line.split(' ');
        let first_token = tokens.next().unwrap();

        if first_token == "$" {
            let command = tokens.next().unwrap();
            if command == "cd" {
                let directory = tokens.next().unwrap().split('/');
                for dir_part in directory {
                    if dir_part == ".." {
                        current_index = directories[current_index].parent.unwrap();
                    }
                    else {
                        current_index = directories[current_index].dirs[dir_part];
                    }
                }
            }
        }
        else {
            if first_token == "dir" {
                directories.push(DirNode {
                    size: 0,
                    parent: Some(current_index),
                    dirs: HashMap::new(),
                });
                let index = directories.len() - 1;
                directories[current_index].dirs.insert(
                    tokens.next().unwrap(),
                    index,
                );
            }
            else {
                directories[current_index].size += first_token
                    .parse::<usize>().unwrap();
            }

        }
    }

    directories
}

fn calculate_size(directories: &mut Vec<DirNode>, current_index: usize) -> usize {
    let dirs = directories[current_index].dirs.clone();
    for (_, directory_index) in dirs {
        let size = calculate_size(directories, directory_index);
        directories[current_index].size += size;
    }

    return directories[current_index].size;
}

fn solve_part1(directories: &Vec<DirNode>) -> usize {
    const MAX_FILE_SIZE: usize = 100_000;

    directories
        .iter()
        .map(|dir| dir.size)
        .filter(|size| *size <= MAX_FILE_SIZE)
        .sum()
}

fn solve_part2(directories: &Vec<DirNode>) -> usize {
    const DISK_SPACE: usize = 70_000_000;
    const NEEDED_SPACE:usize = 30_000_000;
    let space_to_free = NEEDED_SPACE - (DISK_SPACE - directories[0].size);

    directories
        .iter()
        .map(|dir| dir.size)
        .filter(|size| *size >= space_to_free)
        .sorted()
        .next().unwrap()
}

fn main() {
    let mut directories = parse_input(include_str!("../in.txt"));
    calculate_size(&mut directories, 0);

    println!("part1: {}", solve_part1(&directories));
    println!("part2: {}", solve_part2(&directories));
}