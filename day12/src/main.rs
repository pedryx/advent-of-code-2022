use std::collections::HashSet;
use itertools::Itertools;

const PART1: usize = 0;
const PART2: usize = 1;

struct Heightmap {
    heights: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Heightmap {
    fn parse(input: &str) -> Heightmap {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let heights = input.lines()
            .enumerate()
            .map(|(y, l)| l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                        return 'a' as u8 - 'a' as u8;
                    }
                    else if c == 'E' {
                        end = (x, y);
                        return 'z' as u8 - 'a' as u8;
                    }

                    c as u8 - 'a' as u8
                }).collect_vec())
            .collect_vec();

        Heightmap { heights, start, end }
    }

    fn get_height(&self, (x, y): (usize, usize)) -> u8 { self.heights[y][x] }

    fn get_adject_tiles(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut adject = Vec::new();        
        if x > 0                         { adject.push((x - 1, y    )); }
        if y > 0                         { adject.push((x    , y - 1)); }
        if x < self.heights[0].len() - 1 { adject.push((x + 1, y    )); }
        if y < self.heights.len() - 1    { adject.push((x    , y + 1)); }
        adject
    }

    fn solve(&mut self, part: usize) -> usize {
        let mut visited = HashSet::from([self.end]);
        let mut current_wave = vec![self.end];
        let mut next_wave = vec![];
        let mut steps = 0;

        loop {
            steps += 1;
            for current in current_wave.drain(..) {
                if (part == PART1 && current == self.start) ||
                   (part == PART2 && self.get_height(current) == 0)
                {
                    return steps - 1;
                }

                for adject in self.get_adject_tiles(current) {
                    if visited.contains(&adject) ||
                       self.get_height(current) > self.get_height(adject) + 1
                    {
                        continue;
                    }
    
                    visited.insert(adject);
                    next_wave.push(adject);
                }
            }
            std::mem::swap(&mut current_wave, &mut next_wave);    
        }
    }
}

fn main() {
    let mut heightmap = Heightmap::parse(include_str!("../in.txt"));

    println!("part1: {}", heightmap.solve(PART1));
    println!("part2: {}", heightmap.solve(PART2));
}