use itertools::Itertools;
use num::integer::lcm;
use std::cell::RefCell;

const SIMULATED_ROUNDS_PART1: usize = 20;
const SIMULATED_ROUNDS_PART2: usize = 10_000;
const PART1: usize = 0;
const PART2: usize = 1;

struct Monkey
{
    items: [Vec<i64>; 2],
    inspect_count: [usize; 2],
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    monkey_on_pass: usize,
    monkey_on_fail: usize,
}

fn parse_operation(input: &'static str) -> Box<dyn Fn(i64) -> i64> {
    let (operation, value) = input.split_once("old").unwrap().1.trim().split_once(' ').unwrap();

    Box::new(move |input| {
        let value = if value == "old" { input } else { value.parse().unwrap() };

        match operation {
            "+" => input + value,
            "-" => input - value,
            "*" => input * value,
            "/" => input / value,
            _ => panic!("Invalid input!"),
        }
    })
}

fn parse_monkey(input: &'static str) -> Monkey
{
    let (_, starting_items, operation, test, test_pass, test_fail) = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .next_tuple().unwrap();
    let items = starting_items.split(',').map(|item| item.trim().parse().unwrap()).collect_vec();

    Monkey { 
        items: [items.clone(), items],
        inspect_count: [0, 0],
        operation: parse_operation(operation),
        test: test.split(' ').last().unwrap().parse().unwrap(), 
        monkey_on_pass: test_pass.split(' ').last().unwrap().parse().unwrap(), 
        monkey_on_fail: test_fail.split(' ').last().unwrap().parse().unwrap(), 
    }
}

fn simulate_monkey<F>(
    monkeys: &Vec<RefCell<Monkey>>,
    monkey: &RefCell<Monkey>,
    manage_stress: F,
    part: usize
)
where
    F: Fn(i64) -> i64
{
    let item_count = monkey.borrow().items[part].len();
    monkey.borrow_mut().inspect_count[part] += item_count;
    {
        let monkey = monkey.borrow();
        for item in &monkey.items[part] {
            let worry_level = manage_stress((monkey.operation)(*item));
            let target_monkey = if worry_level % monkey.test == 0 {
                monkey.monkey_on_pass
            }
            else {
                monkey.monkey_on_fail
            };
    
            monkeys[target_monkey].borrow_mut().items[part].push(worry_level);
        }
    }
    monkey.borrow_mut().items[part].clear();
}

fn get_result(monkeys: &Vec<RefCell<Monkey>>, part: usize) -> usize {
    let (first_most_active, second_most_active) = monkeys.iter()
        .map(|monkey| monkey.borrow().inspect_count[part])
        .sorted()
        .rev().take(2)
        .next_tuple().unwrap();

    first_most_active * second_most_active
}

fn main() {
    let monkeys = include_str!("../in.txt")
        .split("\n\n")
        .map(|input| RefCell::new(parse_monkey(input)))
        .collect_vec();

    let divisor = monkeys.iter()
        .map(|monkey| monkey.borrow().test)
        .reduce(|a, b| lcm(a, b)).unwrap();

    for round in 0..SIMULATED_ROUNDS_PART2 {
        for monkey in &monkeys {
            if round < SIMULATED_ROUNDS_PART1 {
                simulate_monkey(&monkeys, monkey, |stress| stress / 3, PART1);
            }
            simulate_monkey(&monkeys, monkey, |stress| stress % divisor, PART2);
        }
    }

    println!("part1: {}", get_result(&monkeys, PART1));
    println!("part2: {}", get_result(&monkeys, PART2));
}