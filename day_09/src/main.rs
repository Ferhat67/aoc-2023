use std::{path::Path, fs::File, io::Read};

use itertools::Itertools;

fn main() {
    let input_string = read_input_file("./src/input.txt");
    let histories = parse_histories(input_string);

    let sum_of_extrapolated_values = part1(&histories);
    assert_eq!(sum_of_extrapolated_values, 1969958987);
    println!("Part 1: {}", sum_of_extrapolated_values);

    let sum_of_backwards_extrapolated_values = part2(&histories);
    assert_eq!(sum_of_backwards_extrapolated_values, 1068);
    println!("Part 2: {}", sum_of_backwards_extrapolated_values);

    /**********************
     * Output:
     * Part 1: 1969958987
     * Part 2: 1068
     **********************/
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

fn parse_histories(input_string: String) -> Vec<Vec<i32>> {
    input_string.lines()
        .map(|line| line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>()
}

fn get_diffsequences_for_histories(histories: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    histories.iter()
        .map(|history| {
            let mut current_diffseq = history.clone(); // add the history itself as first diffseq
            let mut diffseqs_for_history = vec![current_diffseq.clone()];
            while !current_diffseq.iter().all(|&b| b == 0) { // until the diffseq consists of zeroes
                current_diffseq = current_diffseq.iter().tuple_windows().map(|(a,b)| b - a).collect_vec(); // pairwise diff
                diffseqs_for_history.push(current_diffseq.clone());
            }
            diffseqs_for_history
        })
        .collect()
}

fn part1(histories: &Vec<Vec<i32>>) -> i32 {
    get_diffsequences_for_histories(histories).iter()
        .map(|diffseqs| {
            // sum up last entry of each sequence
            diffseqs.iter()
                .map(|seq| seq.iter().last().unwrap())
                .sum::<i32>()
        })
        .sum()
}

fn part2(histories: &Vec<Vec<i32>>) -> i32 {
    get_diffsequences_for_histories(histories).iter()
        .map(|diffseqs| {
            // reversely iterate sequences and subtract extrapolated value from the first entry
            diffseqs.iter().rev().fold(0, |a: i32, b: &Vec<i32>| b[0] - a)
        })
        .sum()
}
