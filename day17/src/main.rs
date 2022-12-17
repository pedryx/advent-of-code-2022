use std::collections::HashSet;

type Chunk = u32;
type Dirs = Vec<char>;

const CHUNK_HEIGHT: usize = (std::mem::size_of::<Chunk>() * 8) / CHAMBER_WIDTH;
const LEFT_MASK: Chunk = 0b10000001000000100000010000000000;
const RIGHT_MASK: Chunk = 0b00000010000001000000100000010000;
const HEIGHT_MASK: Chunk = 0b11111110000;

const LEFT: char = '<';
const RIGHT: char = '>';

const SHAPES: [Chunk; 5] = [
    0b00000000000000000000000111100000, // _
    0b00000000001000001110000010000000, // +
    0b00000000000100000010000111000000, // L
    0b00100000010000001000000100000000, // I
    0b00000000000000001100000110000000, // o
];

const SPAWN_OFFSET: usize = 3;
const CHAMBER_WIDTH: usize = 7;
const ROCK_COUNT_PART1: usize = 2022;
const ROCK_COUNT_PART2: usize = 1_000_000_000_000;

const STATE_WINDOW_SIZE: usize = 5;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    window: [Chunk; STATE_WINDOW_SIZE],
    shape_index: usize,
    dir: char,
}

fn solve(dirs: &Dirs, rock_count: usize) -> usize {
    let mut tower: Vec<Chunk> = vec![HEIGHT_MASK, 0];
    let mut tower_height = 0;
    let mut dirs_index: usize = 0;

    let mut states = HashSet::new();
    let mut first_rep_state = State { window: [0; STATE_WINDOW_SIZE], dir: LEFT, shape_index: 0 };
    let mut first_rep_found = false;
    let mut second_rep_found = false;
    let mut first_rep_rock_index = 0;
    let mut first_rep_tower_height = 0;
    let mut skipped_height = 0;

    // process each rock
    let mut it = (0..rock_count).into_iter();
    while let Some(rock_index) = it.next() {
        let mut last_rock = SHAPES[rock_index % SHAPES.len()];
        let mut current_height = tower_height + SPAWN_OFFSET + 1;

        let mut last_top_chunk = 0;
        let mut last_bottom_chunk = 0;
        let mut last_chunk_index = 0;

        // simulate falling
        'steps_loop: for step in 0.. {
            let mut rock = last_rock;
            if step % 2 == 0 {
                // rock is beign pushed by gas
                let current_dir = dirs[dirs_index % dirs.len()];
                dirs_index += 1;

                // move to left if there is no wall
                if current_dir == LEFT && rock & LEFT_MASK == 0 {
                    rock <<= 1;
                }
                // move to eight if there is no wall
                else if current_dir == RIGHT && rock & RIGHT_MASK == 0 {
                    rock >>= 1;
                }
            }
            else {
                // rock falling one unit down
                current_height -= 1;
            }

            // if allocated height is not height enough allocate new chunk
            let allocated_tower_height = tower.len() * CHUNK_HEIGHT;
            if current_height + 2 * CHUNK_HEIGHT > allocated_tower_height {
                tower.push(0);
            }

            // check for collisions
            let chunk_index = current_height / CHUNK_HEIGHT;
            let chunk_offset = current_height % CHUNK_HEIGHT;
            let bottom_chunk = rock << (chunk_offset * CHAMBER_WIDTH);
            let top_chunk = rock >> ((CHUNK_HEIGHT - chunk_offset) * CHAMBER_WIDTH);
            if (bottom_chunk & tower[chunk_index]) == 0 &&
               (top_chunk & tower[chunk_index + 1]) == 0
            {
                // no collision occured so continue into next step
                last_rock = rock;
                last_bottom_chunk = bottom_chunk;
                last_top_chunk = top_chunk;
                last_chunk_index = chunk_index;
                continue;
            }

            // collision occured but we are not moving down so ignore it
            if step % 2 == 0 {
                continue;
            }

            // we need to revert height to the last step
            current_height += 1;

            // store shape into the tower
            tower[last_chunk_index] |= last_bottom_chunk;
            tower[last_chunk_index + 1] |= last_top_chunk;

            // we are trying to find repetetive pattern here
            if !second_rep_found &&
                last_chunk_index > STATE_WINDOW_SIZE
            {
                // compute current state
                let mut window = [0; STATE_WINDOW_SIZE];
                for i in 0..STATE_WINDOW_SIZE {
                    window[i] = tower[last_chunk_index + 1 - i];
                }
                let current_state = State {
                    window: window,
                    dir: dirs[dirs_index % dirs.len()],
                    shape_index: rock_index % SHAPES.len(),
                };

                /*
                    if we found repeating state we store its data, after this we need to find it one
                    more time
                */
                if !first_rep_found {
                    if states.contains(&current_state) {
                        first_rep_rock_index = rock_index;
                        first_rep_tower_height = tower_height;
                        first_rep_state = current_state;
                        first_rep_found = true;
                    }
                    else {
                        states.insert(current_state);
                    }
                }
                /*
                    we now found the repeating state two time so we now computes the diffs and move
                    after last repeat
                */
                else if first_rep_state == current_state {
                    second_rep_found = true;

                    let rock_diff = rock_index - first_rep_rock_index;
                    let height_diff = tower_height - first_rep_tower_height;
                    let repeat_count = (rock_count - rock_index) / rock_diff;
                    it.nth(repeat_count * rock_diff - 1);
                    skipped_height = height_diff * repeat_count;
                }
            }

            // compute new tower height
            for height in (0..CHUNK_HEIGHT).rev() {
                let mask = HEIGHT_MASK << (height * CHAMBER_WIDTH);

                if tower[last_chunk_index + 1] & mask != 0 {
                    tower_height = tower_height.max((last_chunk_index + 1) * CHUNK_HEIGHT + height);
                    break 'steps_loop;
                }
            }
            for height in (0..CHUNK_HEIGHT).rev() {
                let mask = HEIGHT_MASK << (height * CHAMBER_WIDTH);
                if tower[last_chunk_index] & mask != 0 {
                    tower_height = tower_height.max(last_chunk_index * CHUNK_HEIGHT + height);
                    break 'steps_loop;
                }
            }
        }   
    }

    tower_height + skipped_height
}

fn main() {
    let input = include_str!("../in.txt").chars().collect::<Vec<_>>();

    let result1 = solve(&input, ROCK_COUNT_PART1);
    println!("part1: {}", result1);

    let result2 = solve(&input, ROCK_COUNT_PART2);
    println!("part2: {}", result2);
}