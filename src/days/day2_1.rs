use std::fs;

fn main() {
    // Read the text from the text file
    let contents = fs::read_to_string("input/day2/input.txt").expect("file not found");

    // Split by commas
    let ranges = contents.split(",").collect::<Vec<&str>>();

    let mut invalid_numbers: Vec<usize> = vec![];

    for range in ranges {
        let (from, to) = range.split_once('-').unwrap();
        let from_num: usize = from.trim().parse().unwrap();
        let to_num: usize = to.trim().parse().unwrap();

        // Range over the numbers in that range
        for n in from_num..=to_num {
            // Check each numbers length is % 2, then split evenly and check equality
            let num_str: String = n.to_string();

            if num_str.len() % 2 != 0 {
                continue;
            }

            let split_at = num_str.len() / 2;
            let (left, right) = num_str.split_at(split_at);
            if left == right {
                invalid_numbers.push(n);
            }
        }
    }

    // Sum all the values in the invalid_numbers vec
    let sum = invalid_numbers.iter().fold(0, |acc, n| acc + n);

    println!("{:?}", invalid_numbers);
    println!("{}", sum);
}