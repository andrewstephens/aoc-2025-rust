use std::fs::read_to_string;

fn main() {
    let mut password_zero_count = 0;
    let mut starting_number = 50;

    for line in read_lines("input/day1/input1.txt") {
        // Need to split the line to get the direction and number
        let (dir, number) = line.split_at(1);
        let number = number.parse::<i32>().unwrap();

        starting_number = match dir {
            "L" => (starting_number - number) % 100,
            "R" => (starting_number + number) % 100,
            _ => starting_number,
        };

        if starting_number == 0 {
            password_zero_count += 1;
        }
    }

    println!("{}", password_zero_count);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
