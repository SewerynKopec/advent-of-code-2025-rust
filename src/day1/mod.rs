use std::str::Lines;


const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}

pub fn part1() {
    let lines = get_file_lines();

    let mut dial = Dial::init();
    println!("Dial position: {}", dial.get_dial_position());

    for line in lines  {
        println!("Dial instruction: {}", line);
        let raw_rotation_number = &line[1..];
        let rotation_number: i32 = raw_rotation_number.parse().expect("Unable to parse the line into an int");

        if line.chars().nth(0) == Some('R') {
            dial.rotate_right(rotation_number);
        }
        if line.chars().nth(0) == Some('L') {
            dial.rotate_left(rotation_number);
        }
        println!("Dial position: {}", dial.get_dial_position());
    }

    println!("Number of zeros {}", dial.get_zero_counter());
}

pub fn part2() {
    let lines = get_file_lines();

    let mut dial = Dial::init();
    println!("Dial position: {}", dial.get_dial_position());

    for line in lines  {
        println!("Dial instruction: {}", line);
        let raw_rotation_number = &line[1..];
        let rotation_number: i32 = raw_rotation_number.parse().expect("Unable to parse the line into an int");

        if line.chars().nth(0) == Some('R') {
            dial.rotate_right(rotation_number);
        }
        if line.chars().nth(0) == Some('L') {
            dial.rotate_left(rotation_number);
        }
        println!("Dial position: {}", dial.get_dial_position());
        println!("Number of passed zeros {}\n", dial.get_passed_zeros_counter());
    }

    println!("Number of passed zeros {}", dial.get_passed_zeros_counter());
}


struct Dial {
    dial_position: i32,
    dial_range: i32,
    zero_counter: i32,
    passed_zeros_counter: i32,
}

impl Dial {

    fn init() -> Self {
        Self{dial_position: 50, dial_range: 100, zero_counter: 0, passed_zeros_counter: 0}
    }

    fn rotate_left(&mut self, turn_value: i32) {
        let initial_position = self.dial_position;

        self.dial_position -= turn_value;
        while self.dial_position < 0 {
            self.dial_position += self.dial_range;
            self.passed_zeros_counter += 1;
        }

        if initial_position == 0 {
            self.passed_zeros_counter -= 1;
        }

        if self.dial_position == 0 {
            self.zero_counter += 1;
            self.passed_zeros_counter += 1;
        }
    }

    fn rotate_right(&mut self, turn_value: i32) {
        self.dial_position += turn_value;
        while self.dial_position >= self.dial_range {
            self.dial_position -= self.dial_range;
            if self.dial_position != 0  {
                self.passed_zeros_counter += 1;
            }
        }

        if self.dial_position == 0 {
            self.zero_counter += 1;
            self.passed_zeros_counter += 1;
        }
    }

    fn get_dial_position(&self) -> i32 {
        self.dial_position
    }

    fn get_zero_counter(&self) -> i32 {
        self.zero_counter
    }

    fn get_passed_zeros_counter(&self) -> i32 {
        self.passed_zeros_counter
    }
}



