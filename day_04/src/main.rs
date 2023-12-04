use std::{path::Path, fs::File, io::Read};

use regex::Regex;

fn main() {
    let input_string = read_input_file("./src/input.txt");
    let cards = parse_cards(&input_string);

    let total_points = part1(&cards);
    println!("Part 1: {:?}", total_points);
    assert_eq!(total_points, 22897);

    let total_scratchcards = part2(&cards);
    println!("Part 2: {:?}", total_scratchcards);
    assert_eq!(total_scratchcards, 5095824);

    /*******************
     * Output:
     * Part 1: 22897
     * Part 2: 5095824
     *******************/
}

fn read_input_file(input_file_path: &str) -> String {
    let input_path = Path::new(input_file_path);
    let mut input_file = match File::open(&input_path) {
        Err(error) => panic!("Failed to open {} - error: {}", input_path.display(), error),
        Ok(file) => file        
    };
    let mut input_string = String::new();
    match input_file.read_to_string(&mut input_string) {
        Err(error) => panic!("Failed to read contents of {} - error: {}", input_path.display(), error),
        Ok(_bytelength) => println!("Successfully read input file {}", input_path.display())
    }
    input_string
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
    matches: Vec<usize>,
    matchcount: usize
}

fn parse_cards(input_string: &str) -> Vec<Card> {
    let card_regex = Regex::new(r"^Card[ ]+(\d+): ([ \d]*) \| ([ \d]*)$").unwrap();
    let mut cards = vec![];
    for line in input_string.lines() {
        let captures = card_regex.captures(line).unwrap();
        let card_id: usize = captures.get(1).and_then(|id| id.as_str().parse().ok()).unwrap();
        let winning_numbers_string = captures.get(2).and_then(|nums| Some(nums.as_str())).unwrap();
        let winning_numbers = parse_numbers(winning_numbers_string);
        let numbers_string = captures.get(3).and_then(|nums| Some(nums.as_str())).unwrap();
        let numbers = parse_numbers(numbers_string);
        let matches = numbers.iter().filter(|&&num| winning_numbers.contains(&num)).map(|f| f.clone()).collect::<Vec<_>>();
        let matchcount = matches.len();
        cards.push(Card { id: card_id, winning: winning_numbers, numbers, matches, matchcount})
    }
    cards
}

fn parse_numbers(numbers_string: &str) -> Vec<usize> {
    numbers_string
        .split(" ")
        .filter(|&f| !f.is_empty()) // filter out "" elements caused by double-whitespaces in the input data
        .map(|f| f.parse::<usize>().unwrap())
        .collect()
}

fn part1(cards: &Vec<Card>) -> usize {
    cards
        .iter()
        .filter(|&c| c.matchcount > 0)
        .map(|c| usize::pow(2, (c.matchcount - 1) as u32))
        .sum()
}

fn part2(cards: &Vec<Card>) -> usize {
    let mut card_count_by_id = vec![1; cards.len()];
    cards
        .iter()
        .enumerate()
        .map(|(id, card)| (id, card.matchcount))
        .map(|(id, matchcount)| (id, id+1, id+matchcount))
        .map(|(id, next_id, last_id)| (id, (next_id..=last_id)))
        .for_each(|(id, next_ids)| {
            // increase the count of each next-N-cards by the number of cards for current id
            next_ids.into_iter().for_each(|next_id| card_count_by_id[next_id] += card_count_by_id[id]);
        });
    card_count_by_id.iter().sum()
}
