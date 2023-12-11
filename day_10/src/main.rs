pub mod tile;

use std::{fs::read_to_string, collections::VecDeque};

use itertools::Itertools;
use tile::Tile;

fn main() {
    let input_string = read_to_string("./src/input.txt").unwrap();
    let mut grid = parse_tile_grid(&input_string); // TODO: really need to be mutable?

    let part1 = part1(&mut grid);
    let part2 = part2(&mut grid);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 7063);
    assert_eq!(part2, 589);

    /***************
     * Output:
     * Part 1: 7063
     * Part 2: 589
     ***************/
}

fn parse_tile_grid(input_string: &str) -> Vec<Vec<Tile>> {
    input_string.lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| build_tile(c, (x, y)))
                .collect_vec()
        })
        .collect_vec()
}

fn build_tile(pipe: char, position: (usize,usize)) -> Tile {
    Tile {
        position,
        pipe,
        is_start_position: pipe == 'S'
    }
}

fn part1(grid: &mut Vec<Vec<Tile>>) -> usize {
    // S could be any pipe, so we need to test all possibilities
    // find_loop returns empty vector when no loop could be found. Should be ok since we are interested in the largest vector?
    // Possible edge case: two or more different loops depending on start shape? Not relevant for given input...
    "|-F7JL".chars()
        .map(|start_shape| find_loop(grid, start_shape))
        .max_by_key(|loop_positions| loop_positions.len())
        .map(|loop_positions| loop_positions.len()/2) // farthest point is half way through the loop
        .unwrap()
}

fn find_loop(grid: &mut Vec<Vec<Tile>>, start_shape: char) -> Vec<(usize,usize)> {
    let mut queue = VecDeque::new();
    let mut visited_positions = vec![];
    
    let start_tile: &mut Tile = grid.iter_mut().flatten().find(|tile| tile.is_start_position).unwrap();
    start_tile.pipe = start_shape;
    queue.push_back(start_tile.position);

    while !queue.is_empty() {
        let (x,y) = queue.pop_front().unwrap();
        let tile = &grid[x][y];
        visited_positions.push(tile.position);
        
        let west = grid.get(x-1).and_then(|row| row.get(y));
        let east = grid.get(x+1).and_then(|row| row.get(y));
        let north = grid.get(x).and_then(|row| row.get(y-1));
        let south = grid.get(x).and_then(|row| row.get(y+1));
        let connected_tiles = [west, east, north, south].into_iter()
            .filter_map(|other_tile| other_tile)
            .filter(|other_tile| tile.is_connected(&other_tile))
            .collect_vec();
        if connected_tiles.len() != 2 {
            // every pipe in the loop should be connected to exactly two other pipes
            visited_positions.clear();
            break;
        }
        for connected_tile in connected_tiles {
            if !visited_positions.contains(&connected_tile.position) {
                queue.push_back(connected_tile.position);
            }
        }
    }
    visited_positions
}

fn part2(grid: &mut Vec<Vec<Tile>>) -> usize {
    "|-F7JL".chars()
        .map(|start_shape| find_loop(grid, start_shape))
        .max_by_key(|loop_positions| loop_positions.len())
        .map(|loop_positions| {
            // for each tile that is not part of the loop...
            grid.iter()
                .flatten()
                .map(|tile| tile.position)
                .filter(|p| !loop_positions.contains(p))
                .map(|(x,y)| {
                    // count how many crossings (|,J,L) are to the left of it ...
                    (0..y)
                        .map(|i| &grid[x][i])
                        // TODO: speed up by preparing a grid that only contains the loop...
                        .filter(|t| loop_positions.contains(&t.position))
                        .filter(|t| vec!['|','J','L'].contains(&t.pipe))
                        .count()
                })
                // when crossings are an odd number, the tile must be inside the loop
                .filter(|&crossings| crossings % 2 == 1)
                .count()
        })
        .unwrap()
}
