use std::char;

use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(3, 2025).expect("failed to fetch input");

    let mut sum = 0;

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();

        let mut max = 0;

        for (index, char) in chars.iter().enumerate() {
            for other_index in index + 1..chars.len() {
                let other_char = chars[other_index];
                let concated_number = format!("{}{}", char, other_char);
                let number = concated_number.parse::<u32>().unwrap();

                if number > max {
                    max = number;
                }
            }
        }

        sum += max;
    }

    println!("{}", sum);

    Ok(())
}
