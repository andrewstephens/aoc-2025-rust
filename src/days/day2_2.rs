use std::fs;

fn main() {
    // Read the text from the text file
    let contents = fs::read_to_string("input/day2/input.txt").expect("file not found");

    // Split by commas
    let ranges = contents.split(",").collect::<Vec<&str>>();

    // Collect the invalid numbers
    let mut invalid_numbers: Vec<usize> = vec![];

    for range in ranges {
        let (from, to) = range.split_once('-').unwrap();
        let from_num: usize = from.trim().parse().unwrap();
        let to_num: usize = to.trim().parse().unwrap();

        // Range over the numbers in that range
        for n in from_num..=to_num {
            // Check each numbers length is % 2, then split evenly and check equality
            let num_str: String = n.to_string();

            if let Some(_) = find_repeat_index(&num_str) {
                (invalid_numbers).push(n);
            }
        }
    }

    // Sum all the values in the invalid_numbers vec
    let sum = invalid_numbers.iter().fold(0, |acc, n| acc + n);

    println!("{}", sum);
}

// Shamelessly stolen/converted from a python solution
// https://stackoverflow.com/questions/29481088/how-can-i-tell-if-a-string-repeats-itself-in-python
fn find_repeat_index(s: &str) -> Option<&str> {
    let doubled = format!("{}{}", s, s);
    let search_range = &doubled[1..doubled.len() - 1];

    match search_range.find(s) {
        Some(i) => Some(&s[..i + 1]),
        None => None,
    }
}