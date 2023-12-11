use std::{fs::read_to_string, collections::HashMap};

use itertools::Itertools;
use petgraph::{Graph, Directed, algo::floyd_warshall};

fn main() {
    let input_string = read_to_string("./src/input.txt").unwrap();
    let mut image = parse(&input_string);

    let part1 = part1(&mut image);
    let part2 = part2(&mut image);
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 10173804);
    assert_eq!(part2, 634324905172);

    /************************
     * Output:
     * Part 1: 10173804
     * Part 2: 634324905172
     ************************/
}

fn parse(input_string: &str) -> Vec<Vec<char>> {
    input_string.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn part1(image: &Vec<Vec<char>>) -> usize {
    let galaxy_positions = find_galaxies(&image);
    let expansion = get_cosmic_expansion(image);
    let expanded_galaxy_positions = expand_galaxy_positions(&galaxy_positions, expansion, 2);
    let paths = get_paths_between_galaxies(&expanded_galaxy_positions);
    paths.iter()
        .map(|v| distance(v[0], v[1]))
        .sum()
}

fn part2(image: &Vec<Vec<char>>) -> usize {
    let galaxy_positions = find_galaxies(&image);
    let expansion = get_cosmic_expansion(image);
    let expanded_galaxy_positions = expand_galaxy_positions(&galaxy_positions, expansion, 1000000);
    let paths = get_paths_between_galaxies(&expanded_galaxy_positions);
    paths.iter()
        .map(|v| distance(v[0], v[1]))
        .sum()
}

fn find_galaxies(image: &Vec<Vec<char>>) -> Vec<(usize,usize)>{
    let mut galaxies = vec![];
    for x in 0..image.len() {
        for y in 0..image[x].len() {
            if image[x][y] == '#' {
                galaxies.push((x,y));
            }
        }
    }
    galaxies
}

fn get_cosmic_expansion(image: &Vec<Vec<char>>) -> (Vec<usize>,Vec<usize>) {
    let expanding_x = image.iter()
        .enumerate()
        .filter(|&(_,row)| row.iter().all(|&c| c == '.')) // empty rows
        .map(|(x,_)| x)
        .collect_vec();
    let expanding_y = image[0].iter()
        .enumerate()
        .filter(|&(y,_)| image.iter().all(|row| row[y] == '.')) // empty columns
        .map(|(y,_)| y)
        .collect_vec();
    (expanding_x, expanding_y)
}

fn expand_galaxy_positions(galaxy_positions: &Vec<(usize,usize)>, expansion: (Vec<usize>,Vec<usize>), expansion_factor: usize) -> Vec<(usize,usize)> {
    let (expanding_x, expanding_y) = expansion;
    galaxy_positions.iter()
        .map(|(galaxy_x,galaxy_y)| {
            let expansion_x = expanding_x.iter().filter(|&x| x < galaxy_x).count() * (expansion_factor-1);
            let expansion_y = expanding_y.iter().filter(|&y| y < galaxy_y).count() * (expansion_factor-1);
            (galaxy_x + expansion_x, galaxy_y + expansion_y)
        })
        .collect_vec()
}

fn get_paths_between_galaxies(galaxies: &Vec<(usize,usize)>) -> Vec<Vec<(usize,usize)>> {
    galaxies.iter()
        .copied()
        .permutations(2)
        .map(|permutation| permutation.iter().copied().sorted().collect_vec())
        .unique() // after having sorted permutation vectors, we can eliminate duplicates like [(1,2),(3,4)] and [(3,4),(1,2)]
        .collect_vec()
}

fn distance(a: (usize,usize), b: (usize,usize)) -> usize {
    b.0.abs_diff(a.0) + b.1.abs_diff(a.1)
}

// so useless... but its already written so lets keep it around for next puzzles
fn _solve_with_floyd_warshall(nodes: &Vec<(usize,usize)>, edges: &Vec<Vec<(usize,usize)>>) -> i64 {
    let mut graph: Graph<(), i64, Directed> = Graph::new();
    let mut node_index_map = HashMap::new();
    for node in nodes {
        let index = graph.add_node(());
        node_index_map.insert(node, index);
    }
    for edge in edges {
        let start_node = node_index_map[&edge[0]];
        let end_node = node_index_map[&edge[1]];
        let weight = distance(edge[0], edge[1]) as i64;
        graph.add_edge(start_node, end_node, weight);
    }
    let distance_map = floyd_warshall(&graph, |edge| *edge.weight()).unwrap();
    let distances = edges.iter().map(|edge| {
        let a = node_index_map[&edge[0]];
        let b = node_index_map[&edge[1]];
        distance_map[&(a,b)]
    })
    .collect_vec();
    distances.iter().sum()
}
