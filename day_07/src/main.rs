mod card;
mod hand;

use std::{fs::File, io::Read, path::Path};

use hand::Hand;
use itertools::Itertools;

fn main() {
    let input_string = read_input_file("./src/input.txt");
    let mut hands = parse_hands(&input_string);

    let total_winnings = part1(&hands);
    assert_eq!(total_winnings, 251806792);
    println!("Part 1: {}", total_winnings);

    let total_winnings_with_joker_ruke = part2(&mut hands);
    assert_eq!(total_winnings_with_joker_ruke, 252113488);
    println!("Part 2: {}", total_winnings_with_joker_ruke);

    /*********************
     * Output:
     * Part 1: 251806792
     * Part 2: 252113488
     *********************/
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

fn parse_hands(input_string: &str) -> Vec<Hand> {
    input_string.lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|s| (s[0],s[1]))
        .map(|(labels,bid)| Hand::new(labels.chars(), bid.parse::<usize>().unwrap()))
        .collect::<Vec<Hand>>()
}

fn part1(hands: &Vec<Hand>) -> usize {
    hands.iter()
        .sorted() // sorted by strength see impl of Hand and Card structs
        .enumerate()
        .map(|(rank, hand)| hand.get_bid() * (rank+1))
        .sum::<usize>()
}

fn part2(hands: &mut Vec<Hand>) -> usize {
    hands.iter_mut().for_each(|hand| hand.enable_joker_rule()); // toggle joker mode
    hands.iter()
        .sorted() // sorted by strength see impl of Hand and Card structs
        .enumerate()
        .map(|(rank, hand)| hand.get_bid() * (rank+1))
        .sum::<usize>()
}
