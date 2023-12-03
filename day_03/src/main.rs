use std::{path::Path, fs::File, io::Read};

fn main() {
    let input_string = read_input_file("./src/input.txt");

    let (mut schematic, symbol_positions) = parse_schematic_and_symbol_positions(&input_string);
    println!("Part 1: {}", part1(&mut schematic, &symbol_positions));

    // restore the schematic since partnumbers are consumed by part 1
    let (mut schematic, symbol_positions) = parse_schematic_and_symbol_positions(&input_string);
    println!("Part 2: {}", part2(&mut schematic, &symbol_positions));

    /********************
     * Output:
     * Part 1: 536576
     * Part 2: 75741499
     ********************/
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

fn parse_schematic_and_symbol_positions(input_string: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut symbol_positions = vec![];
    let mut schematic_grid = vec![];
    for (x, line) in input_string.lines().enumerate() {
        let mut row = vec![];
        for (y, c) in line.chars().enumerate() {
            row.push(c);
            if !c.is_numeric() && c != '.' {
                symbol_positions.push((x,y));
            }
        }
        schematic_grid.push(row);
    }
    (schematic_grid, symbol_positions)
}

fn part1(schematic: &mut Vec<Vec<char>>, symbol_positions: &Vec<(usize,usize)>) -> usize {
    let mut partnumber_sum: usize = 0;
    for &symbol_position in symbol_positions {
        let adjacent_partnumbers = find_adjacent_partnumbers_for_symbol(schematic, symbol_position);
        partnumber_sum += adjacent_partnumbers.iter().sum::<usize>();
    }
    partnumber_sum
}

fn part2(schematic: &mut Vec<Vec<char>>, symbol_positions: &Vec<(usize,usize)>) -> usize {
    let mut gear_ratio: usize = 0;
    for &(x, y) in symbol_positions {
        if schematic[x][y] == '*' {
            let adjacent_partnumbers = find_adjacent_partnumbers_for_symbol(schematic, (x, y));
            if adjacent_partnumbers.len() > 1 {
                // "A gear is any * symbol that is adjacent to exactly two part numbers." 
                gear_ratio += adjacent_partnumbers.iter().product::<usize>();
            }
        }
    }
    gear_ratio
}

fn find_adjacent_partnumbers_for_symbol(schematic: &mut Vec<Vec<char>>, symbol_position: (usize, usize)) -> Vec<usize> {
    let (x,y) = symbol_position;
    let adjacent_positions: Vec<(usize,usize)> = vec![
        (x, y-1),     // left
        (x, y+1),     // right
        (x-1, y),     // top
        (x+1, y),     // bottom
        (x-1, y-1),   // top-left
        (x-1, y+1),   // top-right
        (x+1, y-1),   // bottom-left
        (x+1, y+1),   // bottom-right
    ]
    .into_iter()
    .filter(|&position| is_position_within_bounds_of_grid(position, schematic.clone()))
    .collect::<Vec<(usize, usize)>>(); 

    let mut partnumbers = vec![];
    for (a,b) in adjacent_positions {
        if schematic[a][b].is_numeric() {
            let partnumber = read_and_consume_partnumber_at(schematic, (a,b));
            partnumbers.push(partnumber);
        }
    }
    partnumbers
}

fn is_position_within_bounds_of_grid(position: (usize,usize), grid: Vec<Vec<char>>) -> bool {
    let (x,y) = position;
    grid.get(x).is_some_and(|row| row.get(y).is_some())
}

fn read_and_consume_partnumber_at(schematic: &mut Vec<Vec<char>>, partnumber_position: (usize, usize)) -> usize {
    let (x,y) = partnumber_position;
    let mut partnumber = schematic[x][y].to_string();
    schematic[x][y] = '.';
    // find, append, consume digits to the right
    for i in y+1..schematic[x].len() {
        if schematic[x][i].is_numeric() {
            partnumber.push(schematic[x][i]);
            schematic[x][i] = '.';
            continue;
        }
        break;
    }
    // find, prepend, consume digits to the left
    for i in (0..y).rev() {
        if schematic[x][i].is_numeric() {
            partnumber = format!("{}{}", schematic[x][i], partnumber);
            schematic[x][i] = '.';
            continue;
        }
        break;
    }
    partnumber.parse().unwrap()
}
