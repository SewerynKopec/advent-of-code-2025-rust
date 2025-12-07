use std::{str::Lines};

const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let battery_banks = get_file_lines();
    let number_of_batteries = 2;
    let max_joltages = get_max_joltages(battery_banks, number_of_batteries);
    let sum_of_joltages: i64 = max_joltages.iter().sum();
    println!("Total joltage: {}", sum_of_joltages);
}

pub fn part2() {
    let battery_banks = get_file_lines();
    let number_of_batteries = 12;
    let max_joltages = get_max_joltages(battery_banks, number_of_batteries);
    let sum_of_joltages: i64 = max_joltages.iter().sum();
    println!("Total joltage: {}", sum_of_joltages);
}

fn get_max_joltages(battery_banks: Lines, number_of_batteries: usize) -> Vec<i64> {
    let mut max_joltages = Vec::<i64>::new();
    for bank in battery_banks {
        println!("Current bank: {}", bank);
        let mut bank_as_vec: Vec<i64> = bank.chars().map(|joltage| joltage.to_digit(10).unwrap() as i64).collect();
        max_joltages.push(get_max_joltage(&mut bank_as_vec, number_of_batteries, None));
        println!("Joltage: {}", max_joltages.last().unwrap());
    }
    return max_joltages;
}

fn get_max_joltage(bank: &mut Vec<i64>, number_of_batteries: usize, buffer: Option<i64>) -> i64 {
    if number_of_batteries == 0 {
        return buffer.unwrap_or(0);
    }

    let max_index = bank.len() - number_of_batteries ;
    let (battery, index) = find_battery(&bank, max_index);
    bank.drain(0..=index);

    let new_buffer = Some(buffer.unwrap_or(0) * 10 + battery);
    return get_max_joltage(bank, number_of_batteries - 1, new_buffer);
}

fn find_battery(batteries: &Vec<i64>, max_index: usize) -> (i64, usize) {
    let mut max_battery = 0;
    let mut current_battery_index = 0;
    let mut max_battery_index = 0;
    for i in 0..=max_index {
        if batteries[i] > max_battery {
            max_battery = batteries[i];
            max_battery_index = current_battery_index;
        }
        current_battery_index += 1;
    }
    return (max_battery, max_battery_index as usize);
}
