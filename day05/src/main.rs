use itertools::Itertools;

fn main() {
    let (towers_in, instructions) = include_str!("../in.txt")
        .split("\n\n")
        .tuples()
        .map(|(towers, instructions)|
            (
                towers.chars()
                    .enumerate()
                    .skip(1).filter(|(i, _)| (i - 1) % 4 == 0)
                    .map(|(_, c)| c)
                    .collect::<Vec<_>>(),
                instructions.lines()
                    .map(|l| l.split(' ')
                        .enumerate()
                        .filter(|(i, _)| i % 2 == 1)
                        .map(|(_, s)| s.parse::<usize>().unwrap())
                        .next_tuple::<(usize, usize, usize)>().unwrap()
                    )
            )
        ).next().unwrap();

    let width: usize = towers_in.last().unwrap().to_digit(10).unwrap() as usize;
    let height = towers_in.len() / width;
    let mut transposed = vec![' '; towers_in.len()];
    transpose::transpose(&towers_in, &mut transposed, width, height);

    let mut towers1 = transposed.chunks(height)
        .map(|chunk| 
            chunk.iter()
            .filter(|c| **c != ' ' && !c.is_ascii_digit())
            .rev()
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let mut towers2 = towers1.clone();

    for (count, from, to) in instructions.clone() {
        for _ in 0..count {
            let item = towers1[from - 1].pop().unwrap();
            towers1[to - 1].push(item);
        }
    }

    for (count, from, to) in instructions {
        let mut cargos: Vec<&char> = Vec::new();
        for _ in 0..count {
            cargos.push(towers2[from - 1].pop().unwrap());
        }
        for cargo in cargos.iter().rev() {
            towers2[to - 1].push(cargo);
        }
    }

    print!("part1: ");
    for tower in towers1 {
        print!("{}", tower.last().unwrap());
    }
    println!();

    print!("part2: ");
    for tower in towers2 {
        print!("{}", tower.last().unwrap());
    }
    println!();
}
