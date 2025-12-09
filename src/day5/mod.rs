use std::{str::Lines};


const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct IdRange {
    first: i64,
    last: i64
}


pub fn part1() {
    let lines = get_file_lines();
    let (fresh_ids, available_ingredients) = parse_lines(lines);
    let fresh_available_ingredients = count_fresh_ingredients(available_ingredients, fresh_ids);
    println!("Fresh ingredients: {fresh_available_ingredients}");
}

pub fn part2() {
    let lines = get_file_lines();
    let (fresh_ids, _) = parse_lines(lines);
    let number_of_fresh_ingredients = count_fresh_ids(fresh_ids);
    println!("Num of fresh ingredients: {number_of_fresh_ingredients}");
}

fn parse_lines(lines: Lines) -> (Vec<IdRange>, Vec<i64>) {
    let lines_vec: Vec<&str> = lines.collect();
    let delimiter_index = lines_vec.iter().position(|line| line.chars().count() == 0).unwrap();

    let mut ranges = Vec::<IdRange>::new();
    let mut ids = Vec::<i64>::new();

    for line in &lines_vec[..delimiter_index] {
        let line_parsed: Vec<i64> = line.split('-').map(|str_number| str_number.parse::<i64>().unwrap()).collect();
        ranges.push(
            IdRange { 
                first: line_parsed[0],
                last: line_parsed[1]
            }
        );
    }
    for line in &lines_vec[(delimiter_index + 1)..] {
        let line_parsed = line.parse::<i64>().unwrap();
        ids.push(line_parsed);
    }
    return (ranges, ids);
}

fn count_fresh_ingredients(available_ingredients: Vec<i64>, fresh_id_ranges: Vec<IdRange>) -> u64 {
    let mut counter = 0;
    for ingredient_id in available_ingredients {
        for range in &fresh_id_ranges {
            if ingredient_id < range.first {
                continue;
            }
            if ingredient_id > range.last {
                continue;
            }
            counter += 1;
            break;
        }
    }

    return counter 
}

fn count_fresh_ids(fresh_id_ranges: Vec<IdRange>) -> i64 {
    let mut sorted_ranges: Vec<IdRange> = fresh_id_ranges.clone();
    sorted_ranges.sort_by(|current, next| current.first.cmp(&next.first));
    let mut max_value = i64::MIN;
    let mut sum = 0;
    let mut iterator = sorted_ranges.iter();
    loop {
        let (first, last) = match iterator.next() {
            Some(x) => {(x.first, x.last)},
            None => {break;},
        };

        // |---------------|
        //         |------|
        if last < max_value {
            continue;
        }

        // |-----|
        //         |------|
        if max_value < first {
            max_value = last;
            sum += last - first + 1;
            continue;
        }

        // |------------|
        //          |------------|
        let moved_start = max_value + 1;
        sum += last - moved_start + 1;
        max_value = last;
    }
    return sum
}
