use std::{collections::{BTreeMap, HashSet}, str::Lines};

use num::iter::Range;

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
    let mut fresh_id_ranges_set = BTreeMap::<i64,i64>::new();
    for range in fresh_id_ranges {
        fresh_id_ranges_set.insert(range.first, range.last);
    }
    let mut keys = fresh_id_ranges_set.keys().peekable();
    let mut unique_fresh_id_ranges = BTreeMap::<i64,i64>::new();
    loop {
        let &current_key = match keys.next() {
            Some(x) => {x},
            None => {break},
        };
        let &next_key = match keys.peek() {
            Some(x) => {*x},
            None => {
                unique_fresh_id_ranges.insert(current_key, fresh_id_ranges_set[&current_key]);
                break
            },
        };

        let value = if fresh_id_ranges_set[&current_key] > next_key {
            keys.next();
            fresh_id_ranges_set[&next_key]
        } else {
            fresh_id_ranges_set[&current_key]
        };
        unique_fresh_id_ranges.insert(current_key, value);
    }
    let mut sum = 0;
    for (first, last) in unique_fresh_id_ranges {
        let local_sum = last - first + 1;
        println!("Range: {first}-{last} => {local_sum}");
        sum += local_sum;
    }
    return sum
}

fn count_fresh_ids_old(fresh_id_ranges: Vec<IdRange>) -> i64 {
    let mut counted_ranges = HashSet::<IdRange>::new();
    println!("Counting..");
    for new_range in fresh_id_ranges {
        let mut ranges_to_remove = HashSet::<IdRange>::new();
        let mut ranges_to_add = HashSet::<IdRange>::new();
        println!("New range {}-{}", new_range.first, new_range.last);
        for &counted_range in &counted_ranges {
            //no overlap, skip the counted range
            if new_range.first > counted_range.last || new_range.last < counted_range.first {
                // println!("No overlap");
                continue;
            }
            println!("Range {}-{}", counted_range.first, counted_range.last);
            // new range overlaped by counted range, skip the new range
            if new_range.first > counted_range.first && new_range.last < counted_range.last {
                println!("New is fully overlapped");
                break;
            }
            // new range fully overlaps the counted range, remove the counted range
            if new_range.first < counted_range.first && new_range.last > counted_range.last {
                println!("New fully overlaps");
                ranges_to_remove.insert(counted_range);
                continue;
            }
            // new range overlaps from left, extend the existing range
            if new_range.first < counted_range.first {
                println!("Overlaps front");
                ranges_to_remove.insert(counted_range);
                ranges_to_add.insert(IdRange { first: new_range.first, last: counted_range.last });
                continue;
            }
            // new range overlaps from right, extend the existing range
            if new_range.last > counted_range.last {
                println!("Overlaps back");
                ranges_to_remove.insert(counted_range);
                ranges_to_add.insert(IdRange { first: counted_range.first, last: new_range.last });
                continue;
            }
        }
        for range_to_remove in ranges_to_remove {
            counted_ranges.remove(&range_to_remove);
        }
        for range_to_add in ranges_to_add {
            counted_ranges.insert(range_to_add);
        }
        counted_ranges.insert(new_range);
    }

    let mut count = 0_i64;
    for range in counted_ranges {
        count += range.last - range.first + 1;
    }
    return count;
}
