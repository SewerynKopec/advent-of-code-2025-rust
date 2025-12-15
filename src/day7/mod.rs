use std::{collections::HashSet, str::Lines};


const FILE_CONTENT: &str = include_str!("./input.txt");

const MANIFOLD: char = 'S';
const SPLITTER: char = '^';

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let lines = get_file_lines();
    let grid = parse_lines(lines);
    let number_of_splits = propagate_beams(grid);
    println!("Number of splits: {number_of_splits}");
}

fn parse_lines(lines: Lines) -> Vec<Vec<char>> {
    let vec_of_lines: Vec<&str> = lines.collect();
    let grid: Vec<Vec<char>> = vec_of_lines.iter().map(|line| line.chars().collect()).collect();
    return grid;
}

fn propagate_beams(grid: Vec<Vec<char>>) -> i32 {
    let mut split_count = 0;
    let mut beam_tracker = HashSet::<usize>::new();
    for row in grid {
        if beam_tracker.is_empty() {
            let manifold_position = row.iter().position(|&element| element == MANIFOLD).expect("No manifold found.");
            beam_tracker.insert(manifold_position);
            continue;
        }
        for (index, cell) in row.iter().enumerate() {
            if beam_tracker.contains(&index) == false {
                continue;
            }
            if *cell == SPLITTER {
                split_count += 1;
                beam_tracker.remove(&index);
                beam_tracker.insert(index - 1);
                beam_tracker.insert(index + 1);
            }
        }
    }
    split_count
}
