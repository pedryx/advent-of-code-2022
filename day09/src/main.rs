use std::collections::HashSet;

use rusttype::Vector;

fn main() {
    let input = include_str!("../in.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, distance)| (
            match direction {
                "U" => Vector { x:  0, y:  1 },
                "L" => Vector { x: -1, y:  0 },
                "D" => Vector { x:  0, y: -1 },
                "R" => Vector { x:  1, y:  0 },
                _ => panic!("Invalid input!"),
            },
            distance.parse::<i32>().unwrap()
        ));

    let mut visited1: HashSet<Vector<i32>> = HashSet::new();
    let mut visited2: HashSet<Vector<i32>> = HashSet::new();
    let mut rope = vec![Vector::<i32> { x: 0, y: 0 }; 10];

    for (direction, distance) in input {
        for _ in 0..distance {
            rope[0] = rope[0] + direction;

            for i in 1..rope.len() {
                let diff = rope[i - 1] - rope[i];
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    rope[i].x += diff.x.signum();
                    rope[i].y += diff.y.signum();
                }
            }
            visited1.insert(rope[1]);
            visited2.insert(*rope.last().unwrap());
        }
    }

    println!("part1: {:?}", visited1.len());
    println!("part2: {:?}", visited2.len());
}