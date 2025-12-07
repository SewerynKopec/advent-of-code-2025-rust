use std::str::Lines;


const FILE_CONTENT: &str = include_str!("./input.txt");
const PAPER_ROLL_CHAR: char = '@';

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let lines = get_file_lines();
    let grid = get_lines_as_grid(lines);
    let number_of_accesable_rolls = count_accesable_rolls(grid);
    println!("Accesable rolls: {}", number_of_accesable_rolls);
}

fn get_lines_as_grid(lines: Lines<'_>) -> Vec<Vec<char>> {
    let mut lines_as_vec = Vec::<Vec<char>>::new();
    for line in lines {
        let line_as_vec: Vec<char> = line.chars().collect();
        lines_as_vec.push(line_as_vec);
    }
    return lines_as_vec
}

fn count_accesable_rolls(grid: Vec<Vec<char>>) -> i32 {
    let num_of_rows = grid.len();
    let num_of_cols = grid.first().unwrap().len();
    let mut num_of_accesable_rolls = 0;
    for row in 0..num_of_rows {
        for col in 0..num_of_cols {
            if grid[row][col] != PAPER_ROLL_CHAR {
                print!(".");
                continue;
            }
            let mut num_of_rolls_around = 0;
            // println!("Around [{}][{}]:", row, col);
            for r in 0..3 {
                let row_offset = r as i32 - 1;
                if row as i32 + row_offset < 0 || row as i32 + row_offset  >= num_of_rows as i32 {
                    continue;
                }
                for c in 0..3 {
                    let col_offset = c as i32 - 1;
                    if row_offset == 0 && col_offset == 0 {
                        // print!(" ");
                        continue;
                    }
                    if col as i32 + col_offset < 0 || col as i32 + col_offset  >= num_of_cols as i32 {
                        continue;
                    }
                    // print!("{}", grid[(row as i32 + row_offset) as usize][(col as i32 + col_offset) as usize]);
                    if grid[(row as i32 + row_offset) as usize][(col as i32 + col_offset) as usize] == PAPER_ROLL_CHAR {
                        // println!("\nRolls around [{}][{}]: [{}][{}]", row, col, row as i32 + row_offset, col as i32 + col_offset);
                        num_of_rolls_around += 1;
                    }
                }
                // print!("\n");
            }
            if num_of_rolls_around < 4 {
                num_of_accesable_rolls += 1;
                // print!("{}", num_of_rolls_around);
                print!("x");
                // println!("Accesable roll at: [{}][{}]", row + 1, col + 1 );
            } else {
                // print!("{}", num_of_rolls_around);
                print!("@");
            }
        }
        print!("\n");
    }
    return num_of_accesable_rolls
}

