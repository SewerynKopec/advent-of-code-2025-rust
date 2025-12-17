use std::{collections::{HashMap, HashSet}, str::Lines, sync::atomic::AtomicI64};


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

pub fn part2() {
    let lines = get_file_lines();
    let grid = parse_lines(lines);
    let number_of_timelines = count_timelines(grid);
    println!("Cache hits: {:?}", CACHE_HITS);
    println!("Cache misses: {:?}", CACHE_MISSES);
    println!("Number of timelines: {number_of_timelines}");
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

fn count_timelines(grid: Vec<Vec<char>>) -> i64 {
    let manifold_position = grid.first().expect("Invalid input").iter().position(|&element| element == MANIFOLD).expect("No manifold found.");
    let mut cache = HashMap::<(usize, usize),i64>::new();
    return 1 + count_timelines_recursive(&grid, 1, manifold_position, &mut cache);
}

static CACHE_HITS: AtomicI64 = AtomicI64::new(0);
static CACHE_MISSES: AtomicI64 = AtomicI64::new(0);

fn count_timelines_recursive(grid: &Vec<Vec<char>>, row_index: usize, beam_position: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
    if row_index >= grid.len() - 1 {
        return 1;
    }

    if cache.contains_key(&(row_index, beam_position)) {
        CACHE_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return cache[&(row_index, beam_position)];
    } else {
        CACHE_MISSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    let next_row_index = row_index + 1;

    if grid[next_row_index][beam_position] != SPLITTER {
        return count_timelines_recursive(grid, next_row_index, beam_position, cache);
    }


    let mut sum = 0;

    if beam_position > 0 {
        let left_split = beam_position - 1;
        sum += if cache.contains_key(&(next_row_index, left_split)) {
            CACHE_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            cache[&(next_row_index, left_split)]
        } else {
            CACHE_MISSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            count_timelines_recursive(grid, next_row_index, left_split, cache)
        };
    }

    let right_split = beam_position + 1;
    if right_split < grid.first().unwrap().len() - 1 {
        sum += if cache.contains_key(&(next_row_index, right_split)) {
            CACHE_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            cache[&(next_row_index, right_split)]
        } else {
            CACHE_MISSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            count_timelines_recursive(grid, next_row_index, right_split, cache)
        };
    }


    cache.insert((row_index, beam_position), sum);
    return sum;
}
