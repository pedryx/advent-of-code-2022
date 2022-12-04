use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    const ALPHABET_SIZE: u32 = 26;

    let result1: u32 = include_str!("../in.txt")
        .lines()
        .map(|l| l.chars()
            .chunks(l.len() / 2)
            .into_iter()
            .map(|chunk| chunk.collect::<HashSet<_>>())
            .next_tuple::<(HashSet<_>, HashSet<_>)>().unwrap()
        )
        .map(|(chunk1, chunk2)| (&chunk1 & &chunk2).iter().next().unwrap().clone())
        .map(|c| c as u32 - if c.is_uppercase() {'A' as u32 - ALPHABET_SIZE} else {'a' as u32} + 1)
        .sum();

    let result2: u32 = include_str!("../in.txt")
        .lines()
        .map(|l| l.chars().collect::<HashSet<_>>())
        .tuples()
        .map(|(g1, g2, g3)| (&(&g1 & &g2) & &g3).iter().next().unwrap().clone())
        .map(|c| c as u32 - if c.is_uppercase() {'A' as u32 - ALPHABET_SIZE} else {'a' as u32} + 1)
        .sum();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
}