use std::collections::{HashMap, HashSet};

use itertools::Itertools;

struct Node {
    flow_rate: u16,
    edges: Vec<Edge>,
}

struct Edge {
    _start_node: usize,
    end_node: usize,
}

#[derive(Debug, Clone)]
struct State {
    opened: HashSet<usize>,
    flow_rate: u16,
    minute: u8,
    preassure: u16,
}

impl State {
    fn process_entity(&mut self, entity: &Entity, graph: &Graph) {
        if entity.open {
            self.flow_rate += graph[entity.pos].flow_rate;
            self.opened.insert(entity.pos);
        }
    }

    fn update(&mut self) -> bool {
        self.preassure += self.flow_rate;
        self.minute -= 1;
    
        self.minute == 0
    }
}

#[derive(Clone, Copy)]
struct Entity {
    pos: usize,
    came_from: usize,
    open: bool,
}

const ROOT_NODE: &str = "AA";
const MAX_MINUTE: u8 = 30;

type Graph = Vec<Node>;

fn parse(input: &str) -> (Graph, usize) {
    let mut graph: Graph = Graph::new();
    let mut nodes: HashMap<&str, usize> = HashMap::new();

    let input = input.lines()
        .map(|line| line.split_once("to").unwrap())
        .map(|(first, second)| (
            first.split(' ')
                .enumerate()
                .filter(|(i, _)| *i == 1 || *i == 4)
                .map(|(_, s)| s)
                .next_tuple().unwrap(),
            second.split(',')
                .map(|s| s.split(' ').last().unwrap())
        )).map(|((node, rate), edges)| (
            (
                node,
                rate.split('=')
                    .last().unwrap()
                    .chars()
                    .take_while(|c| *c != ';').join("")
                    .parse().unwrap()
            ),
            edges
        ));

    for ((node, rate), _) in input.clone() {
        nodes.insert(node, graph.len());
        graph.push(Node { flow_rate: rate, edges: Vec::new() });
    }
    for ((node, _), edges) in input {
        for edge in edges {
            graph[nodes[node]].edges.push(Edge {
                _start_node: nodes[node],
                end_node: nodes[edge],
            });
        }
    }

    //println!("{:?}", nodes);
    (graph, nodes[ROOT_NODE])
}

fn traverse(graph: &Graph, human: Entity, elephant: Entity, mut state: State) -> u16 {
    state.process_entity(&human, graph);
    state.process_entity(&elephant, graph);
    if state.update() {
        return state.preassure;
    }

    let mut preassures: Vec<u16> = Vec::new();
    let human_path_count = graph[human.pos].edges.len();
    let elephant_path_count = graph[elephant.pos].edges.len();
    // 1 if open path included else 0
    let human_open = if graph[human.pos].flow_rate != 0 && !human.open && !state.opened.contains(&human.pos) { 1 } else { 0 };
    let elephant_open = if graph[elephant.pos].flow_rate != 0 && !elephant.open && !state.opened.contains(&elephant.pos) { 1 } else { 0 };

    //println!();
    //println!("human path count: {}", human_path_count);
    //println!("elephant path count: {}", elephant_path_count);
    //println!("human open: {}", human_open);
    //println!("elephant open: {}", elephant_open);

    for human_path in 0..human_path_count + human_open {
        for elephant_path in 0..elephant_path_count + elephant_open {
            // improvment: human and elephant cant open same node

            // human and elephant cannot go back because that is inefective
            if human_path != human_path_count {
                let human_next_pos = graph[human.pos].edges[human_path].end_node;
                if human_next_pos == human.came_from && graph[human.pos].edges.len() != 1 {
                    //println!("human cant go back - {} {}", human_path, elephant_path);
                    continue;
                }
            }
            if elephant_path != elephant_path_count {
                //println!("count: {}", elephant_path_count);
                //println!("edges: {}", graph[elephant.pos].edges.len());
                //println!("path: {}", elephant_path);
                //println!("open: {}", elephant_open);
                let elephant_next_pos = graph[elephant.pos].edges[elephant_path].end_node;
                if elephant_next_pos == elephant.came_from && graph[elephant.pos].edges.len() != 1 {
                    //println!("elephant cant go back - {} {}", human_path, elephant_path);
                    continue;
                }
            }

            // prepare new states for human and elephant
            let mut new_human = human;
            if human_open == 1 && human_path == human_path_count {
                new_human.open = true;
                //println!("human open - {}", human_path);
            }
            else {
                new_human.pos = graph[human.pos].edges[human_path].end_node;
                new_human.came_from = human.pos;
                new_human.open = false;
                //println!("human go - {}", human_path);
            }
            let mut new_elephant = elephant;
            if elephant_open == 1 && elephant_path == elephant_path_count {
                new_elephant.open = true;
                //println!("elephant open - {}", elephant_path);
            }
            else {
                //println!("edges: {}", graph[elephant.pos].edges.len());
                //println!("path: {}", elephant_path);
                //println!("open: {}", elephant_open);
                new_elephant.pos = graph[elephant.pos].edges[elephant_path].end_node;
                new_elephant.came_from = elephant.pos;
                new_elephant.open = false;
                //println!("elephant go - {}", elephant_path);
            }

            // human/elephant move
            let preassure = traverse(graph, new_human, new_elephant, state.clone());
            preassures.push(preassure);
        }
    }

    let preassure = *preassures.iter().max().unwrap();
    //println!("road end - {}", preassure);
    preassure
}

fn solve(graph: &Graph, root_node: usize, time: u8) -> u16 {
    let human = Entity { pos: root_node, came_from: root_node, open: false };
    let elephant = Entity { pos: root_node, came_from: root_node, open: false };

    traverse(graph, human, elephant, State {
        opened: HashSet::new(),
        flow_rate: 0,
        preassure: 0,
        minute: time,
    })
}

fn main() {
    let (graph, root_node) = parse(include_str!("../in_test.txt"));

    let result = solve(&graph, root_node, MAX_MINUTE);
    println!("part1: {}", result);
}
