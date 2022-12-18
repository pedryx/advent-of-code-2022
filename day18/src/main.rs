use std::collections::HashSet;
use itertools::Itertools;

type Num = i16;
type Shape = Vec<Vec<Vec<Num>>>;

const CUBE_SIDE_COUNT: Num = 6;

fn get_neighbours(shape: &Shape, (x, y, z): (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let depth = shape.len();
    let height = shape[0].len();
    let width = shape[0][0].len();

    let mut neighbours = Vec::new();

    if x > 0          { neighbours.push((x - 1, y, z)); }
    if x < width - 1  { neighbours.push((x + 1, y, z)); }
    if y > 0          { neighbours.push((x, y - 1, z)); }
    if y < height - 1 { neighbours.push((x, y + 1, z)); }
    if z > 0          { neighbours.push((x, y, z - 1)); }
    if z < depth - 1  { neighbours.push((x, y, z + 1)); }

    neighbours
}

fn add_cube(shape: &mut Shape, (x, y, z): (usize, usize, usize)) {
    let mut count = 0;

    for (x, y, z) in get_neighbours(shape, (x, y, z)) {
        if shape[z][y][x] != -1 {
            shape[z][y][x] -= 1;
            count += 1;
        }
    }

    shape[z][y][x] = CUBE_SIDE_COUNT - count;
}

fn parse(input: &str) -> Shape {
    let input = input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse::<usize>().unwrap() + 1).next_tuple().unwrap());

    let size_x = input.clone().map(|(x, _, _)| x).max().unwrap() + 3;
    let size_y = input.clone().map(|(_, y, _)| y).max().unwrap() + 3;
    let size_z = input.clone().map(|(_, _, z)| z).max().unwrap() + 3;

    let mut shape = vec![vec![vec![-1; size_x]; size_y]; size_z];
    for cube in input {
        add_cube(&mut shape, cube); 
    }

    shape
}

fn solve_part2(shape: &Shape) -> Num {
    let mut stack = Vec::from([(0, 0, 0)]);
    let mut visited = HashSet::from([(0, 0, 0)]);
    let mut surface = 0;    

    while !stack.is_empty() {
        let current = stack.pop().unwrap();   

        for (x, y, z) in get_neighbours(shape, current) {
            let neighbour = (x, y, z);

            if shape[z][y][x] != -1 {
                surface += 1;
            }
            else if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                stack.push(neighbour);
            }
        }
    }

    surface
}

fn main() {
    let shape = parse(include_str!("../in.txt"));

    let result1: Num = shape.iter().flatten().flatten().filter(|n| **n != -1).sum();
    println!("part1: {}", result1);

    let result2 = solve_part2(&shape);
    println!("part2: {}", result2);
}
