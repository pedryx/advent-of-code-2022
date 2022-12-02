use std::collections::HashMap;

fn main() {
    const LOST_SCORE: u32 = 0;
    const DRAW_SCORE: u32 = 3;
    const WIN_SCORE: u32 = 6;
    let selection_score = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    let outcome_score = HashMap::from([
        ("A Z", LOST_SCORE),
        ("B X", LOST_SCORE),
        ("C Y", LOST_SCORE),
        ("A X", DRAW_SCORE),
        ("B Y", DRAW_SCORE),
        ("C Z", DRAW_SCORE),
        ("A Y", WIN_SCORE),
        ("B Z", WIN_SCORE),
        ("C X", WIN_SCORE),
    ]);
    let transform_selection = HashMap::from([
        ("A Z", "A Y"),
        ("B X", "B X"),
        ("C Y", "C Z"),
        ("A X", "A Z"),
        ("B Y", "B Y"),
        ("C Z", "C X"),
        ("A Y", "A X"),
        ("B Z", "B Z"),
        ("C X", "C Y"),
    ]);

    let total_score1: u32 = include_str!("../in.txt")
        .lines()
        .map(|l| outcome_score[l] + selection_score[&l[l.len() - 1..]])
        .sum();

    println!("part1: {}", total_score1);

    let total_score2: u32 = include_str!("../in.txt")
        .lines()
        .map(|l| transform_selection[l])
        .map(|l| outcome_score[l] + selection_score[&l[l.len() - 1..]])
        .sum();

    println!("part2: {}", total_score2);
}
