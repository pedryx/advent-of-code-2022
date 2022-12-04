use itertools::Itertools;

fn main() {
    let result1 = include_str!("../in.txt")
        .lines()
        .map(|line| line.split(',')
            .map(|range| range.split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .next_tuple().unwrap())
            .next_tuple().unwrap()
        ).filter(|((a, b), (c, d))| (c >= a && d <= b) || (a >= c && b <= d))
        .count();

    let result2 = include_str!("../in.txt")
        .lines()
        .map(|line| line.split(',')
            .map(|range| range.split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .next_tuple().unwrap())
            .next_tuple().unwrap()
        ).filter(|((a, b), (c, d))| a <= d && c <= b)
        .count();

    println!("part1: {}", result1);
    println!("part2: {}", result2);

}
