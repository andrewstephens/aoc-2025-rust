use std::fs::read_to_string;

fn main() {
    let password_zero_count = calculate_zeroes_count();
    println!("{}", password_zero_count);
}

fn calculate_zeroes_count() -> i32 {
    let mut password_zero_count = 0;
    let mut position = 50;

    for line in read_lines("input/day1/input1.txt") {
        let (dir, number_str) = line.split_at(1);
        let steps = number_str.parse::<i32>().expect("Invalid number format");

        let delta = if dir == "L" { -1 } else { 1 };

        for _ in 0..steps {
            position += delta;

            if position == -1 {
                position = 99;
            } else if position == 100 {
                position = 0;
                password_zero_count += 1;
            } else if position == 0 {
                password_zero_count += 1;
            }
        }
    }

    password_zero_count
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect()
}