use std::{path::Path, fs::File, io::Read};

use regex::Regex;

fn main() {
    let part1_result = part1(&read_input_file("./src/input.txt"));
    println!("Part 1: {}", part1_result);
    let part2_result = part2(&read_input_file("./src/input.txt"));
    println!("Part 2: {}", part2_result);

    /******************************************************
     * Output:
     * Part 1: 57346
     * Part 2: 57345
     ******************************************************/
}

// TODO: move to shared utils lib
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

fn part1(input_string: &String) -> u32 {
    let digits_per_line: Vec<u32> = input_string
        .lines()
        .map(|line| extract_digits(line))
        .map(|digits| combine_first_and_last_digits(&digits))
        .collect();
    let total_sum: u32 = digits_per_line.iter().sum();
    total_sum
}

fn extract_digits(line: &str) -> Vec<u32> {
    line.chars().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
}

fn combine_first_and_last_digits(digits: &Vec<u32>) -> u32 {
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<u32>().unwrap()
}

fn part2(input_string: &String) -> u32 {
    let digits_per_line: Vec<u32> = input_string
        .lines()
        .map(|line| insert_numeric_digits_for_spelled_out_digits(line))
        .map(|line| extract_digits(line.as_str()))
        .map(|digits| combine_first_and_last_digits(&digits))
        .collect();
    let total_sum: u32 = digits_per_line.iter().sum();
    total_sum
}

/**
 * Append a numerical digit wherever a spelled-out digit is found.
 * Tricky part is when two spelled-out digits overlap, e.g. "twone".
 * By simply replacing "two" with "2" or "two2" the occurrence of "one" would be destroyed.
 * To avoid that, surround the inserted numeric digit with "#", e.g. "two#2#ne".
 * When doing the search-and-replace just ignore all numeric digits enclosed with #.
 * that way all spelled-out digits and their order of occurrence are preserved.
 * Append numeric digits for simple extraction of first and last digit as was sufficient in Part 1.
 */
fn insert_numeric_digits_for_spelled_out_digits(line: &str) -> String {
    let mut result = line.to_string();
    result = Regex::new(r"o(#[1-9]#)?n(#[1-9]#)?e(#[1-9]#)?").unwrap().replace_all(&result, "$0#1#").to_string();
    result = Regex::new(r"t(#[1-9]#)?w(#[1-9]#)?o(#[1-9]#)?").unwrap().replace_all(&result, "$0#2#").to_string();
    result = Regex::new(r"t(#[1-9]#)?h(#[1-9]#)?r(#[1-9]#)?e(#[1-9]#)?e(#[1-9]#)?").unwrap().replace_all(&result, "$0#3#").to_string();
    // ...
    result = Regex::new(&build_regex_pattern_for_spelled_out_digit("four")).unwrap().replace_all(&result, "$0#4#").to_string();
    result = Regex::new(&build_regex_pattern_for_spelled_out_digit("five")).unwrap().replace_all(&result, "$0#5#").to_string();
    result = Regex::new(&build_regex_pattern_for_spelled_out_digit("six")).unwrap().replace_all(&result, "$0#6#").to_string();
    result = Regex::new(&build_regex_pattern_for_spelled_out_digit("seven")).unwrap().replace_all(&result, "$0#7#").to_string();
    result = Regex::new(&build_regex_pattern_for_spelled_out_digit("eight")).unwrap().replace_all(&result, "$0#8#").to_string();
    result = Regex::new(&build_regex_pattern_for_spelled_out_digit("nine")).unwrap().replace_all(&result, "$0#9#").to_string();
    result
}

fn build_regex_pattern_for_spelled_out_digit(digit_name: &str) -> String {
    digit_name.chars().map(|c| format!("{}{}", c, "(#[1-9]#)?")).collect::<Vec<_>>().join("")
}
