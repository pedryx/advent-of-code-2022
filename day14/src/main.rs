use itertools::Itertools;

#[derive(Clone, Copy)]
struct Point { x: usize, y: usize }

#[derive(Clone, PartialEq)]
enum Tile { Empty, Rock, Sand }

const OFFSET_X: usize = 300;
const OFFSET_FLOOR: usize = 2;
const MAX_WIDTH: usize = 400;
const MAX_HEIGHT: usize = 200;
const GENERATOR: Point = Point { x: 500 - OFFSET_X, y: 0 };
const PART1: u8 = 0;
const PART2: u8 = 1;

type Map = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> (Map, usize) {
    let input = input.lines()
        .map(|l| l.split(" -> ")
            .map(|p| p.split_once(',').unwrap())
            .map(|(x, y)| 
                Point { 
                    x: x.parse::<usize>().unwrap() - OFFSET_X,
                    y: y.parse().unwrap(), 
                }
            )
        );

    let floor_level = input.clone()
        .flat_map(|points| points.map(|p| p.y))
        .max().unwrap() + OFFSET_FLOOR;
    
    let mut map: Map = vec![vec![Tile::Empty; MAX_WIDTH]; MAX_HEIGHT];
    for line in input {
        for (p1, p2) in line.tuple_windows() {
            if p1.x == p2.x {
                let start = std::cmp::min(p1.y, p2.y);
                let end = std::cmp::max(p1.y, p2.y) + 1;
                for y in start..end {
                    map[y][p1.x] = Tile::Rock;
                }
            }
            else {
                let start = std::cmp::min(p1.x, p2.x);
                let end = std::cmp::max(p1.x, p2.x) + 1;
                for x in start..end {
                    map[p1.y][x] = Tile::Rock;
                }
            }
        }
    }

    (map, floor_level)
}

fn simulate(mut map: Map, floor_level: usize, part: u8) -> usize {
    let mut units = 0;

    if part == PART2 {
        for x in 0..MAX_WIDTH {
            map[floor_level][x] = Tile::Rock;
        }
    }

    loop {
        let mut current = GENERATOR;

        'one_unit: loop {
            let points = [
                Point { x: current.x    , y: current.y + 1},
                Point { x: current.x - 1, y: current.y + 1},
                Point { x: current.x + 1, y: current.y + 1},
            ];

            for p in points {
                if part == PART1 && p.y >= MAX_HEIGHT {
                    return units;
                }
                if map[p.y][p.x] == Tile::Empty {
                    current = p;
                    continue 'one_unit;
                }
            }

            map[current.y][current.x] = Tile::Sand;
            break;
        }

        units += 1;
        if part == PART2 && map[GENERATOR.y][GENERATOR.x] == Tile::Sand {
            return  units;
        }
    }
}

fn main() {
    let (map, floor_level) = parse_input(include_str!("../in.txt"));

    let result1 = simulate(map.clone(), floor_level, PART1);
    println!("part1: {}", result1);

    let result2 = simulate(map, floor_level, PART2);
    println!("part2: {}", result2);
}