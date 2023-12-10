use std::{collections::HashMap, time::Instant, u128};

use regex::Regex;

use crate::common::file_io::read_file_to_string;

pub fn puz1() {
    let input = read_file_to_string("input/day8-input");
    let (path, graph) = parse_input(&input);
    let step_count = steps_from_to("AAA", "ZZZ", path, &graph);
    println!("found ZZZ in {step_count} steps");
}

pub fn puz2() {
    let input = read_file_to_string("input/day8-input");
    let (path, graph) = parse_input(&input);

    // find all start and end nodes
    let start_nodes: Vec<String> = graph
        .map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| String::from(s))
        .collect();

    println!("{start_nodes:?}");

    for start_node in start_nodes {
        let count_for_node = steps_from_to(&start_node, "Z", path, &graph);

        println!("Min route for {start_node} is {count_for_node}");
    }
}

const TEST_DATA_3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

pub fn brute() {
    let input = read_file_to_string("input/day8-input");
    let (path, graph) = parse_input(&input);
    let start_nodes: Vec<String> = graph
        .map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| String::from(s))
        .collect();

    let step_count = steps_from_to_para(start_nodes, "Z", path, &graph);

    println!("Min route for all landing at the same time at Z is {step_count}")
}

struct DesertMap {
    map: HashMap<String, (String, String)>,
}

fn parse_input(input: &str) -> (&str, DesertMap) {
    // first line is steps
    let input: Vec<&str> = input.split("\n").collect();

    let steps = input[0].trim();
    let mut map = HashMap::new();

    // for the rest parse the map out
    for line in &input[2..] {
        let halfs: Vec<&str> = line.split("=").collect();

        let node = halfs[0].trim();
        let left_right = halfs[1].trim();
        let left_right = left_right.replace("(", "");
        let left_right = left_right.replace(")", "");
        let left_right: Vec<&str> = left_right.split(",").map(|s| s.trim()).collect();
        let left = left_right[0];
        let right = left_right[1];

        map.insert(
            String::from(node),
            (String::from(left), String::from(right)),
        );
    }
    (steps, DesertMap { map: map })
}

fn steps_from_to_para(from: Vec<String>, to: &str, path: &str, map: &DesertMap) -> u128 {
    let mut step_count: u128 = 0;

    let mut current_node_vec: Vec<&String> = from.iter().map(|f| f).collect();
    println!("{current_node_vec:?}");
    let now = Instant::now();

    while !(current_node_vec.iter().all(|name| name.ends_with(to))) {
        for direction in path.chars() {
            match direction {
                'L' => {
                    current_node_vec = current_node_vec
                        .iter()
                        .map(|name| &map.map.get(*name).unwrap().0)
                        .collect();
                }
                'R' => {
                    current_node_vec = current_node_vec
                        .iter()
                        .map(|name| &map.map.get(*name).unwrap().1)
                        .collect();
                }
                _ => panic!("step that wasn't L or R {direction}"),
            }
            step_count += 1;
        }
        if step_count % 100000 == 0 {
            println!("{step_count}");
            println!("{current_node_vec:?}");
            let elapsed_time = now.elapsed();
            println!(
                "Running slow_function() has taken {} seconds.",
                elapsed_time.as_secs()
            );
        }
    }
    step_count
}

fn steps_from_to(from: &str, to: &str, path: &str, map: &DesertMap) -> u32 {
    let mut step_count = 0;
    let mut current_node = map.map.get(from).unwrap();
    let mut current_node_name = from;

    while current_node_name.ends_with(to) == false {
        for direction in path.chars() {
            match direction {
                'L' => {
                    current_node_name = &current_node.0;
                    current_node = map.map.get(current_node_name).unwrap()
                }
                'R' => {
                    current_node_name = &current_node.1;
                    current_node = map.map.get(current_node_name).unwrap()
                }
                _ => panic!("step that wasn't L or R {direction}"),
            }
            step_count += 1;

            // see if we're at the destination
            if !current_node_name.ends_with(to) == false {
                // reached destination
                break;
            }
        }
    }
    step_count
}
mod test {
    use crate::day8::{steps_from_to, steps_from_to_para, TEST_DATA_3};

    use super::parse_input;

    #[test]
    pub fn test1() {
        let (path, graph) = parse_input(TEST_DATA);
        let step_count = steps_from_to("AAA", "ZZZ", path, &graph);
        assert_eq!(step_count, 2);
    }

    #[test]
    pub fn test2() {
        let (path, graph) = parse_input(TEST_DATA_2);
        let step_count = steps_from_to("AAA", "ZZZ", path, &graph);
        assert_eq!(step_count, 6);
    }

    #[test]
    pub fn test_brute() {
        let (path, graph) = parse_input(TEST_DATA_3);
        let start_nodes: Vec<String> = graph
            .map
            .keys()
            .filter(|s| s.ends_with('A'))
            .map(|s| String::from(s))
            .collect();

        let step_count = steps_from_to_para(start_nodes, "Z", path, &graph);
        assert_eq!(step_count, 6);
    }

    const TEST_DATA: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA_2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
}
