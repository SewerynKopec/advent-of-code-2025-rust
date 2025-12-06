use std::{collections::HashSet, f32, hash::Hash, mem::offset_of, str::Lines};

const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

const RANGE_DELIMITER: char = ',';
const ID_DELIMITER: char = '-';

pub fn part1() {
    let lines = get_file_lines();
    let ranges = get_ranges(lines);
    let mut total_invalid_ids_sum = 0;
    let only_double_occurances = false;
    for range in ranges {
        total_invalid_ids_sum += sum_invalid_ids(range, only_double_occurances);
    }
    println!("\nInvalid ids sum: {}", total_invalid_ids_sum);
}

pub fn part2() {
    let lines = get_file_lines();
    let ranges = get_ranges(lines);
    let mut total_invalid_ids_sum = 0;
    let only_double_occurances = false;
    for range in ranges {
        let sum = sum_invalid_ids(range, only_double_occurances);
        total_invalid_ids_sum += sum;

        println!("Sum so far: {}", total_invalid_ids_sum);
        println!("Last sum:   {}", sum);
    }
    println!("\nInvalid ids sum: {}", total_invalid_ids_sum);
}

struct ID {
    value: u64,
    num_of_digits: usize
}

fn get_ranges(lines: Lines<'_>) -> Vec<(ID,ID)>{
    let mut ranges = Vec::<(ID,ID)>::new();
    for line in lines {
        line.split(RANGE_DELIMITER).for_each( |range| {
            if range.trim() == "" {
                return;
            }
            println!("Range: {}", range);
            let mut range_split = range.split(ID_DELIMITER);

            let (raw_first_id, raw_last_id) = (
                range_split.clone().nth(0).unwrap(),
                range_split.nth(1).unwrap()
            );

            let first_id = ID{
                value: raw_first_id.parse().unwrap(),
                num_of_digits: raw_first_id.chars().count()
            };
            let last_id = ID{
                value: raw_last_id.parse().unwrap(),
                num_of_digits: raw_last_id.chars().count()
            };

            ranges.push((first_id, last_id));
        });
    }
    return ranges
}

fn sum_invalid_ids(input_range: (ID, ID), only_double_occurances: bool) -> u64 {
    if input_range.0.num_of_digits != input_range.1.num_of_digits {
        let base: u32 = 10;
        let range_break: u64 = base.pow(input_range.0.num_of_digits as u32) as u64;
        let range_break_id_left = ID { value: range_break - 1, num_of_digits: input_range.0.num_of_digits };
        let range_break_id_right = ID { value: range_break, num_of_digits: input_range.1.num_of_digits };
        return sum_invalid_ids((input_range.0, range_break_id_left), only_double_occurances) + sum_invalid_ids((range_break_id_right, input_range.1), only_double_occurances);
    }
    if only_double_occurances && input_range.0.num_of_digits % 2 == 1 {
        return 0;
    }

    println!("\n");
    let range = input_range.0.value..(input_range.1.value + 1);

    // 11 % 11 = 0
    // 1212 % 101 = 0
    // 123123 % 1001 = 0
    // 121212 % 10101 = 0

    let size_divisors: HashSet<usize> = if only_double_occurances {
        HashSet::from([2])
    } else {
        get_size_divisors(input_range.0.num_of_digits)
    };

    // let invalid_id_templates: Vec<u64> = size_divisors.iter().map(|&size_divsor| {
    //     return compute_invalid_id_templates(input_range.0.num_of_digits, size_divsor);
    // }).collect();
    // println!("Templates: {:?}",  invalid_id_templates);


    println!("Current range: {:?}", range);
    let mut invalid_ids = HashSet::<u64>::new();
    // let mut invalid_ids_by_string = HashSet::<u64>::new();
    for id in range {
        if is_id_invalid_by_string(&id) {
            invalid_ids.insert(id);
            println!("Invalid id from string: {}", id);
        }

        // invalid_id_templates.iter().for_each(|invalid_template| {
        //     if id % invalid_template == 0 {
        //         println!("Invalid id: {}", id);
        //         invalid_ids.insert(id);
        //     }
        // });
    }
    return invalid_ids.iter().sum();
}

fn is_id_invalid_by_string(id: &u64) -> bool {
    let string_id = id.to_string();
    let id_as_vec: Vec<char> = string_id.chars().collect();
    let string_len = string_id.chars().count() as u64;
    let mut repeating_number_lengths = Vec::new();
    for i in 1..string_len {
        if string_len % i == 0 {
            repeating_number_lengths.push(i as u64);
        }
    }

    // println!("Current id from string: {}", id);
    // println!("Number lenghts to check {:?}", repeating_number_lengths.iter());
    for number_length in repeating_number_lengths {
        // println!("Checked number length: {}", number_length);
        let mut is_every_digit_same = true;
        let number_of_repetitions = string_len / number_length;
        for digit_position in 0..number_length {
            let mut nth_digit_values = HashSet::<char>::new();
            for repetition in 0..number_of_repetitions {
                let index = (repetition * number_length) + digit_position;
                // println!("Current checked index: {}", index);
                nth_digit_values.insert(id_as_vec[index as usize]);
            }
            if nth_digit_values.len() > 1 {
                is_every_digit_same = false;
                break
            }
        }
        if is_every_digit_same {
            return true
        }
    }
    return false;
}

fn get_size_divisors(size: usize) -> HashSet<usize> {
    let mut size_divisors = HashSet::<usize>::new();
    let divisor_check_limit = (size as f32).sqrt() as usize;
    for i in 1..(divisor_check_limit + 1) {
        if size % i == 0 {
            if i != 1 {
                size_divisors.insert(i);
            }
            size_divisors.insert(size / i);
        }
    }
    return size_divisors;
}

fn compute_invalid_id_templates(number_size: usize, divisor: usize) -> u64 {
    //121212 -> 6 numbers / 5 zeros 
    
    //123123 ->   1001
    //121212 ->  10101
    //222222 -> 111111

    let mut template = 1;
    for _ in 0..(divisor - 1) {
        let base: u32 = 10;
        template *= base.pow((number_size / divisor) as u32);
        template += 1;
    }

    return template as u64;
}
