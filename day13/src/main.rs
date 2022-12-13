use itertools::Itertools;
use std::cmp::Ordering;

fn read_num(input: &mut &str) -> u8 {
    let num = input.chars().take_while(|c| c.is_ascii_digit()).join("");
    *input = &mut &input[num.len()..];
    num.parse().unwrap()
}

fn compare_set_num(
    number: &mut &str,
    set: &mut &str,
    reverse: bool
) -> Ordering {
    *set = &set[1..];
    let mut result = compare_sets(number, set);

    if result == Ordering::Equal {
        result = compare_sets(&mut "]", set);
    }
    result = if reverse { result.reverse() } else { result };
    return result;
}

fn compare_sets(left: &mut &str, right: &mut &str) -> Ordering {
    if left.starts_with(",") { 
        *left = &left[1..];
    }
    if right.starts_with(",") {
        *right = &right[1..];
    }

    let first_char_left = left.chars().next().unwrap();
    let first_char_right = right.chars().next().unwrap();

    if first_char_left.is_ascii_digit() && first_char_right.is_ascii_digit() {
        return read_num(left).cmp(&read_num(right));
    }
    else if first_char_left == '[' && first_char_right == '[' {
        *left = &left[1..];
        *right = &right[1..];
        return compare_sets(left, right);
    }
    else if first_char_left.is_ascii_digit() && first_char_right == '[' {
        return compare_set_num(left, right, false);
    }
    else if first_char_left == '[' && first_char_right.is_ascii_digit() {
        return compare_set_num(right, left, true);
    }
    else if first_char_left == ']' && first_char_right == ']' {
        *left = &left[1..];
        *right = &right[1..];
        return Ordering::Equal;
    }
    else if first_char_left == ']' {
        *left = &left[1..];
        return Ordering::Less;
    }
    else {
        *right = &right[1..];
        return Ordering::Greater;
    }
}

fn compare_packets(left: &str, right: &str) -> Ordering {
    let mut left_packet = left;
    let mut right_packet = right;

    loop {
        let result = compare_sets(&mut left_packet, &mut right_packet);
        if result != Ordering::Equal {
            return result;
        }
    }
}

fn main() {
    let result: usize = include_str!("../in.txt")
        .split("\n\n")
        .map(|packets| packets.split_once('\n').unwrap())
        .map(|(left, right)| compare_packets(left, right, ))
        .enumerate()
        .filter(|(_, result)| *result == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum();
    println!("part1: {:?}", result);

    let (div_packet1, div_packet2) = include_str!("../in.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .chain(["[[2]]", "[[6]]"])
        .sorted_by(|a, b| compare_packets(a, b))
        .enumerate()
        .filter(|(_, s)| *s == "[[2]]" || *s == "[[6]]")
        .map(|(i, _)| i + 1)
        .next_tuple().unwrap();
    println!("part2: {}", div_packet1 * div_packet2);
}