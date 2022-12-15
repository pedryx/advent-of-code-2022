use std::collections::HashSet;

use itertools::Itertools;

type Num = i32;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point { x: Num, y: Num }

struct Line { start: Point, len: usize, dir: u8 }

const DESIRED_Y: i32 = 10;
const MIN_LIMIT: i32 = 0;
const MAX_LIMIT: i32 = 4000000;

fn solve_part1(sensors: &Vec<Point>, beacons: &Vec<Point>) -> usize {
    let mut desired_points: HashSet<i32> = HashSet::new();
    for i in 0..sensors.len() {
        let max_range = (sensors[i].x - beacons[i].x).abs() +
                       (sensors[i].y - beacons[i].y).abs();

        let distance = (sensors[i].y - DESIRED_Y).abs();
        if distance > max_range {
            continue;
        }

        let diff = max_range - distance;
        for x in sensors[i].x-diff..sensors[i].x+diff+1 {
            desired_points.insert(x);
        }

        if beacons[i].y == DESIRED_Y {
            desired_points.remove(&beacons[i].x);
        }
    }

    desired_points.len()
}

fn solve_part2(sensors: &Vec<Point>, beacons: &Vec<Point>) -> i64 {
    let mut points: Vec::<Point> = Vec::new();

    for i in 0..sensors.len() {
        println!("{}", i);
        let max_range = (sensors[i].x - beacons[i].x).abs() +
                        (sensors[i].y - beacons[i].y).abs();

        for j in 0..max_range+1 {
            points.push(Point { 
                x: sensors[i].x - max_range - 1 + j,
                y: sensors[i].y - j,
            });
            points.push(Point { 
                x: sensors[i].x + j,
                y: sensors[i].y - max_range - 1 + j,
            });
            points.push(Point { 
                x: sensors[i].x + max_range + 1 - j,
                y: sensors[i].y + j,
            });
            points.push(Point { 
                x: sensors[i].x - j,
                y: sensors[i].y + max_range + 1 - j,
            });
        }
    }

    let size = points.len();
    println!("{}", size);
    let mut count = 0;
    'points_loop: for p in points {
        count += 1;
        if count % 1000000 == 0 {
            println!("{}/{}", count, size);
        }

        if p.x < MIN_LIMIT || p.y < MIN_LIMIT || p.x > MAX_LIMIT || p.y > MAX_LIMIT {
            continue;
        }

        for i in 0..sensors.len() {
            let max_range = (sensors[i].x - beacons[i].x).abs() +
                            (sensors[i].y - beacons[i].y).abs(); // 6 + 3 = 9

            let distance = (p.x - sensors[i].x).abs() +
                           (p.y - sensors[i].y).abs();

            // 8 7
            // 10 - 8 + 7 - 10 = 2 + 3

            if distance <= max_range {
                continue 'points_loop;
            }
        }

        return (p.x as i64) * 4_000_000 + (p.y as i64);
    }

    return 0;
}

fn main() {
    let mut sensors: Vec<Point> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    include_str!("../in.txt")
        .lines()
        .map(|l| l.split('=')
            .skip(1)
            .map(|s| s.chars()
                .enumerate()
                .take_while(|(i, c)| *i == 0 || c.is_ascii_digit())
                .map(|(_, c)| c)
                .collect::<String>()
                .parse::<Num>().unwrap()
            ).next_tuple().unwrap()
        ).for_each(|(sx, sy, bx, by)| {
            sensors.push(Point { x: sx, y: sy });
            beacons.push(Point { x: bx, y: by });
        });

    //let result1 = solve_part1(&sensors, &beacons);
    //println!("part1: {}", result1);
    //assert_eq!(result1, 6_425_133);

    let result2 = solve_part2(&sensors, &beacons);
    println!("part2: {}", result2);
    assert_eq!(result2, 10_996_191_429_555);
}
