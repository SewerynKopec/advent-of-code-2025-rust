use std::{str::Lines};

use num::abs;


type Transform = fn((i64,i64)) -> (i64, i64);

const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let lines = get_file_lines();
    let red_tiles = parse_red_tiles(lines);
    let area = calculate_highest_area_rectangle(red_tiles);
    println!("{}",area);
}

fn parse_red_tiles(lines: Lines) -> Vec<(i64,i64)> {
    let mut red_tiles = Vec::new();
    for line in lines {
        let parsed_line: Vec<&str> = line.split(",").collect();
        let red_tile: (i64, i64) = (parsed_line[0].parse().unwrap(), parsed_line[1].parse().unwrap());
        red_tiles.push(red_tile);
    }
    println!("Red tiles: \n {:?}", red_tiles);
    return red_tiles;
}

fn calculate_highest_area_rectangle(red_tiles: Vec<(i64, i64)>) -> i64 {
    let mut max_area = -1;
    let mut point1: (i64, i64) = (-1, -1);
    let mut point2: (i64, i64) = (-1, -1);
    for (index, first_tile) in red_tiles.iter().enumerate() {
        for second_tile in red_tiles.iter().take(index + 1) {
            let area = (abs(first_tile.1 - second_tile.1) + 1) * (abs(first_tile.0 - second_tile.0) + 1);
            if max_area < area {
                max_area = area;
                point1 = *first_tile;
                point2 = *second_tile;
            }
        }
    }

    println!("{:?}:{:?}", point1, point2);
    max_area
}

