type Num = isize;

const DECRYPTION_KEY: Num = 811_589_153;
const MIX: Num = 10;
const START_VALUE: Num = 0;
const DESIRED_POSITONS: [usize; 3] = [1_000, 2_000, 3_000];

#[derive(Copy, Clone, PartialEq)]
struct Item {
    value: Num,
    id: u16,
}

fn solve(mut original: Vec<Item>, key: Num, mix: Num) -> Num {
    for item in &mut original {
        item.value *= key;
    }
    let mut sequence = original.clone();
    let len = original.len() as Num - 1;

    for _ in 0..mix {
        for item in &original {
            let index = sequence.iter().enumerate().find(|(_, n)| **n == *item).unwrap().0 as Num;
            let destination = ((index + item.value) % len + len) % len;
    
            sequence.remove(index as usize);
            sequence.insert(destination as usize, *item);
        }
    }

    let start_index = sequence.iter().enumerate().find(|(_, n)| n.value == START_VALUE).unwrap().0;
    DESIRED_POSITONS.iter()
        .map(|p| sequence[(p + start_index) % (len as usize + 1)].value)
        .sum::<Num>()
}

fn main() {
    let mut last_id = 0;
    let input = include_str!("../in.txt")
        .lines()
        .map(|l|{
            last_id += 1;
            Item { value: l.parse().unwrap(), id: last_id }
        })
        .collect::<Vec<_>>();

    let result1 = solve(input.clone(), 1, 1);
    println!("part1: {:?}", result1);

    let result2 = solve(input, DECRYPTION_KEY, MIX);
    println!("part2: {:?}", result2);
}