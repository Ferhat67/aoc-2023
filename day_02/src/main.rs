use std::{path::Path, fs::File, io::Read};

use regex::Regex;

fn main() {
    let input_string = read_input_file("./src/input.txt");
    let games: Vec<Game> = input_string.lines().map(|line| parse_game(line)).collect();

    let sum_of_possible_game_ids: u32 = games
        .iter()
        .filter(|&game| is_game_possible(game, 12, 13, 14))
        .map(|game| game.id)
        .sum();
    println!("Part 1: {}", sum_of_possible_game_ids);

    let sum_of_power_of_minimum_cube_sets: u32 = games
        .iter()
        .map(|game| minimum_cubes(game))
        .map(|(red,green,blue)| red * green * blue)
        .sum();
    println!("Part 2: {}", sum_of_power_of_minimum_cube_sets);

    /*****************
     * Output:
     * Part 1: 2551
     * Part 2: 62811
     *****************/
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
struct Game {
    id: u32,
    reveals: Vec<(u32, u32, u32)>
}

fn parse_game(line: &str) -> Game {
    let game_regex = Regex::new(r"Game (\d+): (.*$)").unwrap();
    let game_id: u32 = game_regex.captures(line).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let reveal_strings: Vec<&str> = game_regex.captures(line).unwrap().get(2).unwrap().as_str().split(";").collect();

    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();

    let reveals: Vec<(u32, u32, u32)> = reveal_strings.iter().map(|&reveal_string| {
        let red_cubes: u32 = parse_number_of_cubes(&red_regex, reveal_string);
        let green_cubes: u32 = parse_number_of_cubes(&green_regex, reveal_string);
        let blue_cubes: u32 = parse_number_of_cubes(&blue_regex, reveal_string);
        (red_cubes, green_cubes, blue_cubes)
    }).collect();
    Game { id: game_id, reveals }
}

fn parse_number_of_cubes(regex: &Regex, reveal_string: &str) -> u32 {
    regex
        .captures(reveal_string)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str().parse().unwrap())
        .unwrap_or(0) // if no match is found, color was not given explicitly
}

fn is_game_possible(game: &Game, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    game.reveals.iter().all(|&(red,green,blue)| red <= max_red && green <= max_green && blue <= max_blue)
}

fn minimum_cubes(game: &Game) -> (u32, u32, u32) {
    let min_red = game.reveals.iter().map(|&(red,_,_)| red).max().unwrap();
    let min_green = game.reveals.iter().map(|&(_,green,_)| green).max().unwrap();
    let min_blue = game.reveals.iter().map(|&(_,_,blue)| blue).max().unwrap();
    (min_red, min_green, min_blue)
}
