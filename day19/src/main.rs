use std::collections::HashMap;
use scanf::sscanf;

type Cache = HashMap<State, i8>;

const TIME_PART1: i8 = 24;
const TIME_PART2: i8 = 32;

struct Robot {
    ore_cost: i8,
    clay_cost: i8,
    obsidian_cost: i8,
}

struct Blueprint {
    id: i8,
    ore_bot: Robot,
    clay_bot: Robot,
    obsidian_bot: Robot,
    geode_bot: Robot,
    max_cost_ore: i8,
    max_cost_clay: i8,
    max_cost_obsidian: i8,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    time: i8,
    ore_count: i16,
    clay_count: i16,
    obsidian_count: i16,
    geode_count: i8,
    ore_bots: i8,
    clay_bots: i8,

    obsidian_bots: i8,
    geode_bots: i8,
}

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    fn build(&self, state: &mut State) {
        state.ore_count -= self.ore_cost as i16;
        state.clay_count -= self.clay_cost as i16;
        state.obsidian_count -= self.obsidian_cost as i16;
    }
}

impl Blueprint {
    fn parse(input: &str) -> Blueprint {
        let mut id = 0;
        let mut ore_bot_cost = 0;
        let mut clay_bot_cost = 0;
        let mut obsidian_bot_cost_ore = 0;
        let mut obsidian_bot_cost_clay = 0;
        let mut geode_bot_cost_ore = 0;
        let mut geode_bot_cost_obsidian = 0;

        sscanf!(
            input,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            id,
            ore_bot_cost,
            clay_bot_cost,
            obsidian_bot_cost_ore,
            obsidian_bot_cost_clay,
            geode_bot_cost_ore,
            geode_bot_cost_obsidian,
        ).unwrap();

        Blueprint {
            id,
            ore_bot: Robot {
                ore_cost: ore_bot_cost,
                clay_cost: 0,
                obsidian_cost: 0
            },
            clay_bot: Robot {
                ore_cost: clay_bot_cost,
                clay_cost: 0,
                obsidian_cost: 0
            },
            obsidian_bot: Robot {
                ore_cost: obsidian_bot_cost_ore,
                clay_cost: obsidian_bot_cost_clay,
                obsidian_cost: 0
            },
            geode_bot: Robot {
                ore_cost: geode_bot_cost_ore,
                clay_cost: 0,
                obsidian_cost: geode_bot_cost_obsidian
            },
            max_cost_ore: std::cmp::max(
                std::cmp::max(ore_bot_cost, clay_bot_cost),
                std::cmp::max(obsidian_bot_cost_ore, geode_bot_cost_ore),
            ),
            max_cost_clay: obsidian_bot_cost_clay,
            max_cost_obsidian: geode_bot_cost_obsidian,
        }
    }

    fn calc_quality_level_inner(&self, cache: &mut Cache, mut state: State, to_build: RobotType) -> i8 {
        // update resources
        while state.is_negative() {
            state.update_resources();
            if state.time == 0 {
                return state.geode_count;
            }
        }
        state.update_resources();
        if state.time == 0 {
            return state.geode_count;
        }

        // build robot
        match to_build {
            RobotType::Ore => state.ore_bots += 1,
            RobotType::Clay => state.clay_bots += 1,
            RobotType::Obsidian => state.obsidian_bots += 1,
            RobotType::Geode => state.geode_bots += 1,
        }

        if cache.contains_key(&state) {
            return cache[&state];
        }

        // decide what to build next
        let mut max_geodes = 0;
        if state.ore_bots < self.max_cost_ore {
            let mut new_state = state.clone();
            self.ore_bot.build(&mut new_state);
            max_geodes = max_geodes.max(self.calc_quality_level_inner(cache, new_state, RobotType::Ore));
        }
        if state.clay_bots < self.max_cost_clay {
            let mut new_state = state.clone();
            self.clay_bot.build(&mut new_state);
            max_geodes = max_geodes.max(self.calc_quality_level_inner(cache, new_state, RobotType::Clay));

        }
        if state.obsidian_bots < self.max_cost_obsidian && state.clay_bots > 0 {
            let mut new_state = state.clone();
            self.obsidian_bot.build(&mut new_state);
            max_geodes = max_geodes.max(self.calc_quality_level_inner(cache, new_state, RobotType::Obsidian));
        }
        if state.obsidian_bots > 0 {
            let mut new_state = state.clone();
            self.geode_bot.build(&mut new_state);
            max_geodes = max_geodes.max(self.calc_quality_level_inner(cache, new_state, RobotType::Geode));
        }
        cache.insert(state, max_geodes);

        max_geodes
    }
    
    fn calc_quality_level(&self, time: i8) -> i8 {
        let state = State {
            time: time + 1,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
            ore_bots: 0,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        };
        self.calc_quality_level_inner(&mut HashMap::new(), state, RobotType::Ore)
    }
}

impl State {
    fn update_resources(&mut self) {
        self.ore_count += self.ore_bots as i16;
        self.clay_count += self.clay_bots as i16;
        self.obsidian_count += self.obsidian_bots as i16;
        self.geode_count += self.geode_bots;
        self.time -= 1;
    }

    fn is_negative(&self) -> bool {
        self.ore_count < 0 || self.clay_count < 0 || self.obsidian_count < 0
    }
}

fn main() {
    let input = include_str!("../in.txt")
        .lines()
        .map(Blueprint::parse);

    let result1 = input.clone()
        .map(|b| b.calc_quality_level(TIME_PART1) as i16 * b.id as i16)
        .sum::<i16>();
    println!("part1: {}", result1);

    let result2 = input
        .take(3)
        .map(|b| b.calc_quality_level(TIME_PART2) as i16)
        .product::<i16>();
    println!("part2: {}", result2);
}