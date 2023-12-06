use std::{path::Path, fs::File, io::Read};

fn main() {
    let input_string = read_input_file("./src/input.txt");
    let races = parse_races(&input_string);

    let product_of_number_of_winning_strategies_per_race = part1(&races);
    println!("Part 1: {}", product_of_number_of_winning_strategies_per_race);
    assert_eq!(product_of_number_of_winning_strategies_per_race, 1731600);

    let number_of_winning_strategies = part2(&races);
    println!("Part 2: {}", number_of_winning_strategies);
    assert_eq!(number_of_winning_strategies, 40087680);

    /********************
     * Output:
     * Part 1: 1731600
     * Part 2: 40087680
     ********************/
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

fn parse_races(input_string: &str) -> Vec<(usize, usize)> {
    let lines = input_string.lines().collect::<Vec<_>>();
    let race_durations = lines[0].replace("Time:", "").split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let record_distances = lines[1].replace("Distance:", "").split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    race_durations.into_iter().zip(record_distances.into_iter()).collect::<Vec<_>>()
}

fn part1(races: &Vec<(usize,usize)>) -> usize {
    races.iter().map(|&race| get_number_of_winning_strategies_for_race(race)).product()
}

fn get_number_of_winning_strategies_for_race(race: (usize,usize)) -> usize {
    let (race_duration,record_distance) = race;
    (1..race_duration) // for each charge time that is not 0 and not the entire race duration => boat will move
        .map(|charge_time| {
            let travel_time = race_duration - charge_time;
            let travel_distance = travel_time * charge_time;
            (travel_distance, record_distance)
        })
        .filter(|&(travel_distance, record_distance)| travel_distance > record_distance)
        .count()
}

fn part2(races: &Vec<(usize,usize)>) -> usize {
    // no need to parse again, just concatenate digits
    let race_duration = races.iter().map(|&(t,_)| t.to_string()).collect::<String>().parse::<usize>().unwrap();
    let record_distance = races.iter().map(|&(_,d)| d.to_string()).collect::<String>().parse::<usize>().unwrap();

    get_number_of_winning_strategies_for_race((race_duration, record_distance))
}
