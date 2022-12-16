use std::collections::HashMap;
use itertools::Itertools;
use scanf::sscanf;

type Num = i16;
type Cache = Vec<Num>;
type Graph = Vec<Node>;

struct Node {
    edges: Vec<usize>,
    flow_rate: Num,
}

struct State {
    pos: usize,
    opened: u64,
    time: usize,
}

const ROOT_NODE: &str = "AA";
const MAX_MINUTE_PART1: usize = 30;
const MAX_MINUTE_PART2: usize = 26;

fn parse(input: &str) -> (Graph, usize) {
    let input = input.lines()
        .map(|line| line.split_once(';').unwrap())
        .map(|(left, right)| {
            let mut name: String = String::new();
            let mut flow_rate: Num = 0;
            sscanf!(left, "Valve {} has flow rate={}", name, flow_rate).unwrap();
            let edges = right.split(' ').skip(5).map(|s| s.replace(",", ""));

            (name, flow_rate, edges)
        }).sorted_by(|(_, a, _), (_, b, _)| b.partial_cmp(a).unwrap())
        .enumerate();

    let mut nodes = HashMap::new();
    let mut graph = Graph::new();
    let mut root_node = 0;
    for (i, (name, flow_rate, _)) in input.clone() {
        if name == ROOT_NODE {
            root_node = i;
        }

        nodes.insert(name, graph.len());
        graph.push(Node { flow_rate, edges: Vec::new() });
    }
    for (i, (_, _, edges)) in input {
        for edge in edges {
            graph[i].edges.push(nodes[&edge]);
        }
    }

    (graph, root_node)
}

fn solve_inner(
    graph: &Graph,
    cache: &mut Cache,
    total_time: usize,
    root_node: usize,
    total_helper_count: usize,
    helper_count: usize,
    state: State
) -> Num {
    // if no time left switching to next helper or exiting
    if state.time == 0 {
        if helper_count - 1 > 0 {
            let state = State { pos: root_node, time: total_time, ..state };
            return solve_inner(graph, cache, total_time, root_node, total_helper_count, helper_count - 1, state);
        }
        else {
            return 0;
        }
    }

    // if result is cached get it from cache
    let cache_index = state.opened as usize * total_time  * graph.len() * total_helper_count  +
                      (state.time - 1)  * graph.len() * total_helper_count  +
                      state.pos * total_helper_count  + helper_count;
    if cache[cache_index] >= 0 {
        return cache[cache_index];
    }

    let mut result = 0;

    // try to open if can be opened
    if (state.opened & (1 << state.pos)) == 0 && graph[state.pos].flow_rate > 0 {
        let opened = state.opened | (1 << state.pos);
        let accumulated = (state.time - 1) as Num * graph[state.pos].flow_rate;
        let state = State { opened, time: state.time - 1, ..state };
        result = std::cmp::max(
            result,
            accumulated + solve_inner(graph, cache, total_time, root_node, total_helper_count, helper_count, state)
        );
    }
    // go through each edge
    for edge in &graph[state.pos].edges {
        let state = State { time: state.time - 1, pos: *edge, ..state };
        result = std::cmp::max(
            result,
            solve_inner(graph, cache, total_time, root_node, total_helper_count, helper_count, state)
        );
    }

    // store result in cache
    cache[cache_index] = result;

    result
}

fn solve(graph: &Graph, root_node: usize, total_time: usize, helper_count: usize) -> Num {
    let openable_count = graph.iter().filter(|n| n.flow_rate > 0).count();
    let cache_size = (1 << openable_count) * graph.len() * total_time as usize * helper_count as usize;
    let mut cache: Vec<Num> = vec![-1;cache_size];
    let state = State { pos: root_node, time: total_time, opened: 0 };

    solve_inner(&graph, &mut cache, total_time, root_node, helper_count, helper_count, state)
}

fn main() {
    let (graph, root_node) = parse(include_str!("../in.txt"));

    let result1 = solve(&graph, root_node, MAX_MINUTE_PART1, 1);
    println!("part1: {}", result1);

    let result2 = solve(&graph, root_node, MAX_MINUTE_PART2, 2);
    println!("part2: {}", result2);
}