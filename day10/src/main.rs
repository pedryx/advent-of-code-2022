const NOOP_CYCLES: i32 = 1;
const ADDX_CYCLES: i32 = 2;
const SPRITE_SIZE: i32 = 3;
const LINE_SIZE: i32 = 40;
const OFFSET: i32 = 20;

fn draw_pixel(cycle: i32, sprite_position: i32) {
    if cycle % LINE_SIZE >= sprite_position && cycle % LINE_SIZE < sprite_position + SPRITE_SIZE {
        print!("#");
    }
    else {
        print!(".");
    }

    if cycle % LINE_SIZE == 0 {
        println!();
    }
}

fn process_signal_score(signal_score: &mut i32, cycle: i32, x_value: i32) {
    if (cycle - OFFSET) % LINE_SIZE == 0 {
        *signal_score += cycle * x_value;
    }
}

fn main() {
    let result = include_str!("../in.txt").lines();

    let mut last_cycle = 1;
    let mut last_x = 1;
    let mut signal_score = 0;

    for line in result {
        let current_cycle = last_cycle;
        let current_x = last_x;

        draw_pixel(current_cycle, current_x);
        process_signal_score(&mut signal_score, current_cycle, current_x);

        // addx instruction
        if let Some((_, x_change)) = line.split_once(' ') {
            last_cycle += ADDX_CYCLES;
            last_x += x_change.parse::<i32>().unwrap();

            draw_pixel(current_cycle + 1, current_x);
            process_signal_score(&mut signal_score, current_cycle + 1, current_x);
        }
        // noop instruction
        else {
            last_cycle += NOOP_CYCLES;
        }
    }

    println!("part1: {}", signal_score);
}
