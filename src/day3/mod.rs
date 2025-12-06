use std::{str::Lines};

const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let battery_banks = get_file_lines();
    let max_joltages = get_max_joltages(battery_banks);
    let sum_of_joltages: i32 = max_joltages.iter().sum();
    println!("Total joltage: {}", sum_of_joltages);
}

fn get_max_joltages(battery_banks: Lines) -> Vec<i32> {
    let mut max_joltages = Vec::<i32>::new();
    for bank in battery_banks {
        println!("Current bank: {}", bank);
        let mut bank_as_vec: Vec<u32> = bank.chars().map(|joltage| joltage.to_digit(10).unwrap()).collect();
        let (first_battery, index) = find_battery(&bank_as_vec);
        if index == bank_as_vec.len() - 1 {
            bank_as_vec.pop();
            let (second_battery, _) = find_battery(&bank_as_vec);
            max_joltages.push(10 * second_battery as i32 + first_battery as i32);
            println!("Joltage: {}{}", &second_battery, &first_battery);
            continue;
        }
        bank_as_vec.drain(0..=index);
        let (second_battery, _) = find_battery(&bank_as_vec);
        max_joltages.push(10 * first_battery as i32 + second_battery as i32);
        println!("Joltage: {}{}", &first_battery, &second_battery);
    }
    return max_joltages;
}

fn find_battery(batteries: &Vec<u32>) -> (u32, usize) {
    // return batteries.iter()
    //     .enumerate()
    //     .max_by_key(|&(_, value)| value)
    //     .map(|(index, &value)| (value, index))
    //     .unwrap();
    let mut max_battery = 0;
    let mut current_battery_index = 0;
    let mut max_battery_index = 0;
    for battery in batteries {
        if *battery > max_battery {
            max_battery = *battery;
            max_battery_index = current_battery_index;
        }
        current_battery_index += 1;
    }
    return (max_battery, max_battery_index as usize);
}
