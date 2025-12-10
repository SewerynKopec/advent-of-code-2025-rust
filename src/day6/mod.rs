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
        for factor_row in factors.to_vec() {
            let current_factor: i64 = factor_row[col].as_str().parse().expect("Invalid input data for factors.");
            println!("{current_factor}");
            let operation = wrapped_operation.expect("Invalid input data for operation");

            operation(&mut results[col], current_factor);
        }
    }

    return results;
}
