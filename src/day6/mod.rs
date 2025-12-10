use std::{str::Lines};


const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let lines = get_file_lines();
    let input_grid = parse_lines(lines);
    let results = calculate_columns(input_grid);
    let sum: i64 = results.iter().sum();
    println!("Sum of all the operations: {}", sum);
}

pub fn part2() {
    let lines = get_file_lines();
    let input_grid = split_individual_operations(lines);
    let results = calculate_factors_vertically(input_grid);
    let sum: i64 = results.iter().sum();
    println!("Sum of all the operations: {}", sum);
}

fn parse_lines(lines: Lines) -> Vec<Vec<String>> {
    let mut vec_of_lines = Vec::<Vec<String>>::new();

    for line in lines {
        let mut single_spaces_line = "".to_owned();
        let mut chars = line.chars();
        let mut current_spaces_counter = 0;
        while let Some(current_char) = chars.next() {
            if current_char == ' ' {
                current_spaces_counter += 1;
            } else {
                current_spaces_counter = 0;
            }

            if current_spaces_counter <= 1 {
                single_spaces_line.push(current_char);
            }
        }
        single_spaces_line = single_spaces_line.trim().to_string();
        println!("old line: {line}");
        // println!("new line: {single_spaces_line}");
        let vec_of_words: Vec<String> = single_spaces_line.split(' ').map(|word| word.to_owned()).collect();
        vec_of_lines.push(vec_of_words);
    };
    return vec_of_lines;
}

fn split_individual_operations(lines: Lines) -> Vec<Vec<String>> {
    let mut lines_vec = lines.collect::<Vec<&str>>();
    let (operations, factors) = lines_vec.split_last_mut().expect("Invalid input");

    let mut parsed_lines = vec![Vec::<String>::new(); factors.len() + 1];

    let mut previous_split_index = 0;
    for (index, character) in operations.char_indices() {
        if character == ' ' || index == 0 {
            continue;
        }
        let split_index = index - 1;
        for (factor_line_index, factor_line) in factors.iter().enumerate() {
            parsed_lines[factor_line_index].push(factor_line[previous_split_index..split_index].to_string());
        }
        parsed_lines[factors.len()].push(operations[previous_split_index..split_index].to_string());
        // println!("operation {}", operations[previous_split_index..index].to_string());
        previous_split_index = split_index + 1;
    }
    for (factor_line_index, factor_line) in factors.iter().enumerate() {
        parsed_lines[factor_line_index].push(factor_line[previous_split_index..].to_string());
    }
    parsed_lines[factors.len()].push(operations[previous_split_index..].to_string());

    parsed_lines
}

fn calculate_columns(input_grid: Vec<Vec<String>>) -> Vec<i64> {
    let mut results = Vec::<i64>::new();
    let (operations, factors) = &input_grid.split_last().unwrap();
    for col in 0..input_grid.first().expect("Invalid input rows").len() {
        let current_operation_sign = &operations[col].as_str();
        let wrapped_operation: Option<fn(&mut i64,i64)> = match *current_operation_sign {
            "*" => {
                results.push(1);
                Some(|first, second| *first *= second)
            },
            "+" => {
                results.push(0);
                Some(|first, second| *first += second)
            },
            _ => None
        };
        let operation = wrapped_operation.expect("Invalid input data for operation");
        for factor_row in factors.to_vec() {
            let current_factor: i64 = factor_row[col].as_str().parse().expect("Invalid input data for factors.");
            println!("{current_factor}");

            operation(&mut results[col], current_factor);
        }
    }

    return results;
}

fn calculate_factors_vertically(input_grid: Vec<Vec<String>>) -> Vec<i64> {
    let mut results = Vec::<i64>::new();
    let (operations, factors) = &input_grid.split_last().unwrap();
    let factor_row_len = factors.len();
    println!("operations len {}", operations.len());
    println!("factors len {}", factors.first().unwrap().len());

    let number_of_problems = input_grid.first().expect("Invalid input rows").len();
    for problem in 0..number_of_problems  {
        let current_operation_sign = &operations[problem].as_str().trim();
        // println!("Current operation '{}'", current_operation_sign);
        let wrapped_operation: Option<fn(&mut i64,i64)> = match *current_operation_sign {
            "*" => {
                results.push(1);
                Some(|first, second| *first *= second)
            },
            "+" => {
                results.push(0);
                Some(|first, second| *first += second)
            },
            _ => None
        };

        let operation = wrapped_operation.expect("Invalid input data for operation");
        let string_len = factors.first().unwrap()[problem].chars().count();

        for char_position in 0..string_len {
            let mut buffer = "".to_owned();
            for factor_row in 0..factor_row_len {
                let char_column: Vec<char> = factors[factor_row][problem].chars().collect();
                // println!("char col {:?}", char_column.iter());
                let digit = char_column[char_position];
                // println!("Next digit {}", digit);
                buffer.push(digit);
            }
            let parsed_number = buffer.trim().parse::<i64>().unwrap();
            println!("Parsed number {}\n", parsed_number);
            operation(&mut results[problem], parsed_number);
        };
    }

    return results; 
}
