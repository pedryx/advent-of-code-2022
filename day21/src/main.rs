use std::collections::HashMap;

type Num = i64;
type Monkeys<'a> = HashMap<&'a str, Monkey<'a>>;
type Frac = [Num; 2];
type Polynomial = [Frac; 2];

const ROOT_MONKEY: &str = "root";
const HUMAN: &str = "humn";

enum Monkey<'a> {
    Literal(Num),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn edit_frac(f: Frac) -> Frac {
    let gcd = num::integer::gcd(f[0], f[1]);

    if gcd == 0 {
        f
    }
    else {
        [f[0] / gcd, f[1] / gcd]
    }
}

fn add_frac(a: Frac, b: Frac) -> Frac {
    edit_frac([a[0] * b[1] + a[1] * b[0], a[1] * b[1]])
}

fn sub_frac(a: Frac, b: Frac) -> Frac {
    edit_frac([a[0] * b[1] - a[1] * b[0], a[1] * b[1]])
}

fn mul_frac(a: Frac, b: Frac) -> Frac {
    edit_frac([a[0] * b[0], a[1] * b[1]])
}

fn div_frac(a: Frac, b: Frac) -> Frac {
    edit_frac([a[0] * b[1], a[1] * b[0]])
}

fn add_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut res = [[0, 1]; 2];

    for i in 0..res.len() {
        res[i] = add_frac(a[i], b[i]);
    }
    
    res
}

fn sub_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut res = [[0,1]; 2];

    for i in 0..res.len() {
        res[i] = sub_frac(a[i], b[i]);
    }

    res
}

fn mul_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut res = [[0, 1]; 2];

    res[0] = mul_frac(a[0], b[0]);
    res[1] = add_frac(mul_frac(a[0], b[1]), mul_frac(a[1], b[0]));

    res
}

fn div_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut res = [[0, 1]; 2];

    if a[1] == [0, 0] {
        res[0] = div_frac(b[0], a[0]);
        res[1] = div_frac(b[1], a[0]);
    }
    else {
        res[0] = div_frac(a[0], b[0]);
        res[1] = div_frac(a[1], b[0]);
    }

    res
}

fn parse_input(input: &str) -> Monkeys {
    input.lines()
    .map(|l| {
        let tokens = l.split(' ').collect::<Vec<_>>();

        (
            tokens[0].split(':').next().unwrap(),
            if tokens.len() > 2 {
                match tokens[2] {
                    "+" => Monkey::Add(tokens[1], tokens[3]),
                    "-" => Monkey::Sub(tokens[1], tokens[3]),
                    "*" => Monkey::Mul(tokens[1], tokens[3]),
                    "/" => Monkey::Div(tokens[1], tokens[3]),
                    _ => panic!("invalid input!")
                }
            }
            else {
                Monkey::Literal(tokens[1].parse().unwrap())
            }
        )
    }).collect::<Monkeys>()
}

fn evaluate1(monkeys: &HashMap<&str, Monkey>, monkey: &str) -> Num {
    match monkeys[monkey] {
        Monkey::Literal(n) => n,
        Monkey::Add(m1, m2) => evaluate1(monkeys, m1) + evaluate1(monkeys, m2),
        Monkey::Sub(m1, m2) => evaluate1(monkeys, m1) - evaluate1(monkeys, m2),
        Monkey::Mul(m1, m2) => evaluate1(monkeys, m1) * evaluate1(monkeys, m2),
        Monkey::Div(m1, m2) => evaluate1(monkeys, m1) / evaluate1(monkeys, m2),
    }
}

fn evaluate2(monkeys: &HashMap<&str, Monkey>, monkey: &str) -> Polynomial {
    if monkey == ROOT_MONKEY {
        if let Monkey::Add(m1, m2) = monkeys[monkey] {
            return sub_poly(evaluate2(monkeys, m2), evaluate2(monkeys, m1));
        }
    }
    else if monkey == HUMAN {
        return [[0, 1], [1, 1]];
    }

    match monkeys[monkey] {
        Monkey::Literal(n) => [[n, 1], [0, 1]],
        Monkey::Add(m1, m2) => add_poly(evaluate2(monkeys, m1), evaluate2(monkeys, m2)),
        Monkey::Sub(m1, m2) => sub_poly(evaluate2(monkeys, m1), evaluate2(monkeys, m2)),
        Monkey::Mul(m1, m2) => mul_poly(evaluate2(monkeys, m1), evaluate2(monkeys, m2)),
        Monkey::Div(m1, m2) => div_poly(evaluate2(monkeys, m1), evaluate2(monkeys, m2)),
    }
}

fn main() {
    let monkeys = parse_input(include_str!("../in.txt"));

    let result1 = evaluate1(&monkeys, ROOT_MONKEY);
    println!("part1: {}", result1);

    let result2 = evaluate2(&monkeys, ROOT_MONKEY);
    let num1 = result2[0][0] as f64 / result2[0][1] as f64;
    let num2 = result2[1][0] as f64 / result2[1][1] as f64;
    let result2 = -num1 / num2;
    println!("part2: {}", result2);
}