use itertools::Itertools;

fn main() {
    let data = include_str!("../in1.txt")
        .lines()
        .map(|i| i.trim())
        .group_by(|i| i.is_empty())
        .into_iter()
        .filter(|(k, _)| !k)
        .map(|(_, g)| g.map(|i| i.parse::<u32>().unwrap()).sum::<u32>())
        .sorted();

    println!("part1: {}", data.clone().last().unwrap());
    println!("part2: {}", data.clone().rev().take(3).sum::<u32>());
}