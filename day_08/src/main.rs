use std::{path::Path, fs::File, io::Read, collections::HashMap};

use num::Integer;
use regex::Regex;


fn main() {
    let input_string = read_input_file("./src/input.txt");
    let network = parse_network(&input_string);
    let instructions = parse_instructions(&input_string);

    let steps = part1(&network, &instructions);
    assert_eq!(steps, 21409);
    println!("Part 1: {:?}", steps);

    let steps = part2(&network, &instructions);
    assert_eq!(steps, 21165830176709);
    println!("Part 2: {:?}", steps);

    /**************************
     * Output:
     * Part 1: 21409
     * Part 2: 21165830176709
     **************************/
}

fn read_input_file(input_file_path: &str) -> String {
    let mut input_string = String::new();
    match File::open(Path::new(input_file_path)) {
        Err(error) => panic!("Failed to open {} - error: {}", input_file_path, error),
        Ok(mut file) => match file.read_to_string(&mut input_string) {
            Err(error) => panic!("Failed to read contents of {} - error: {}", input_file_path, error),
            Ok(_bytelength) => println!("Successfully read input file {}", input_file_path)
        }
    }
    input_string
}

fn parse_network(input_string: &str) -> HashMap<&str, Vec<&str>> {
    let regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    input_string.lines()
        .skip(2) // skip the instructions
        .map(|line| regex.captures(line).unwrap().extract::<3>())
        .map(|(_line, nodes)| (nodes[0], vec![nodes[1], nodes[2]]))
        .collect::<HashMap<_,_>>()
}

fn parse_instructions(input_string: &str) -> Vec<usize> {
    input_string.lines()
        .next() // first line
        .map(|line| {
            line.replace("L", "0").replace("R", "1").chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()
        })
        .unwrap()
}

fn part1(network: &HashMap<&str, Vec<&str>>, instructions: &Vec<usize>) -> usize {
    let mut node = "AAA";
    for (step, &instruction) in instructions.iter().cycle().enumerate() {
        node = &network[node][instruction];
        if node == "ZZZ" {
            return step + 1;
        }
    }
    panic!("Should never reach this code...")
}

fn part2(network: &HashMap<&str, Vec<&str>>, instructions: &Vec<usize>) -> usize {
    let start_nodes = network.keys().filter(|node| node.ends_with("A")).cloned().collect::<Vec<_>>();
    let steps_by_start_node = start_nodes.iter().map(|start_node| {
        let mut node = start_node;
        for (step, &instruction) in instructions.iter().cycle().enumerate() {
            node = &network[node][instruction];
            if node.ends_with("Z") {
                return step + 1;
            }
        }
        panic!("Should never reach this code...")
    }).collect::<Vec<_>>();
    steps_by_start_node.iter().fold(1, |a,b| a.lcm(b)) // #total steps = largest common multiplier of all #steps for each start node
}
