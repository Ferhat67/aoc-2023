use std::{path::Path, fs::File, io::Read, ops::Range};

fn main() {
    let input_string = read_input_file("./src/input.txt");
    let almanac = parse_almanac(&input_string);

    let lowest_location_number_for_seeds = part1(&almanac);
    println!("Part 1: {:?}", lowest_location_number_for_seeds);
    assert_eq!(lowest_location_number_for_seeds, 346433842);

    let lowest_location_number_for_seed_ranges = part2(&almanac);
    println!("Part 2: {:?}", lowest_location_number_for_seed_ranges);
    assert_eq!(lowest_location_number_for_seed_ranges, 60294664);

    /*********************
     * Output:
     * Part 1: 346433842
     * Part 2: 60294664
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<(usize,usize,usize)>>
}

fn parse_almanac(input_string: &str) -> Almanac {
    let input_sections = input_string.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_seeds(&input_sections[0]);
    let seed_to_soil_map = parse_map(&input_sections[1]);
    let soil_to_fertilizer_map = parse_map(&input_sections[2]);
    let fertilizer_to_water_map = parse_map(&input_sections[3]);
    let water_to_light_map = parse_map(&input_sections[4]);
    let light_to_temperature_map = parse_map(&input_sections[5]);
    let temperature_to_humidity_map = parse_map(&input_sections[6]);
    let humidity_to_location_map = parse_map(&input_sections[7]);

    Almanac { 
        seeds,
        maps: vec![
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map
        ]
    }
}

fn parse_seeds(section: &str) -> Vec<usize> {
    section
        .replace("seeds: ", "")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_map(section: &str) -> Vec<(usize,usize,usize)> {
    section
        .lines()
        .enumerate()
        .filter(|&(i,_)| i != 0) // skip map title in first line
        .map(|(_,line)| line.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .map(|values| (values[0],values[1],values[2]))
        .collect()
}

fn part1(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|&seed| convert_seed_to_location(seed, almanac))
        .min()
        .unwrap()
}

fn convert_seed_to_location(seed: usize, almanac: &Almanac) -> usize {
    let mut value = seed;
    for map in &almanac.maps {
        for &(destination_range_start, source_range_start, range_length) in map {
            if (source_range_start .. source_range_start + range_length).contains(&value) {
                value = (value - source_range_start) + destination_range_start;
                break; // no need to check other mappings if one did already match
            }
        }
    }
    value
}

fn part2(almanac: &Almanac) -> usize {
    let mut location = 0;
    let ranges = almanac.seeds.chunks(2).map(|chunk| (chunk[0] .. chunk[0]+chunk[1])).collect::<Vec<Range<usize>>>();
    loop {
        let seed = convert_location_to_seed(location, almanac);
        if is_seed_in_ranges(seed, &ranges) {
            break location;
        }
        location += 1;
    }
}

fn convert_location_to_seed(location: usize, almanac: &Almanac) -> usize {
    let mut value = location;
    for map in almanac.maps.iter().rev() {
        for &(destination_range_start, source_range_start, range_length) in map {
            if (destination_range_start .. destination_range_start + range_length).contains(&value) {
                value = (value - destination_range_start) + source_range_start;
                break; // no need to check other mappings if one did already match
            }
        }
    }
    value
}

fn is_seed_in_ranges(seed: usize, ranges: &Vec<Range<usize>>) -> bool {
    ranges.iter().any(|range| range.contains(&seed))
}
