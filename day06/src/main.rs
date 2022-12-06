use std::collections::HashSet;

fn solve(window_length: usize) -> usize {
    return include_str!("../in.txt")
        .as_bytes()
        .windows(window_length)
        .map(|w| w.iter().collect::<HashSet<_>>())
        .enumerate()
        .skip_while(|(_, w)| w.len() != window_length)
        .next().unwrap().0 + window_length;
}

fn main() {
    println!("part1: {}", solve(4));
    println!("part2: {}", solve(14));
}
