use std::collections::HashMap;

use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(1, 2025).expect("failed to fetch input");

    let mut value: i64 = 50;
    let mut combination: i64 = 0;

    let mut cache = HashMap::new();

    let mut rotate = |num: i64| -> () {
        let cache_key = (value, num);

        if let Some(cached_value) = cache.get(&cache_key) {
            value = ((value + num) % 100 + 100) % 100;

            combination += cached_value;

            return;
        }

        let mut combination_iter = 0;

        for _ in 0..num.abs() {
            let sign = if num > 0 { 1 } else { -1 };

            value = ((value + sign) % 100 + 100) % 100;

            if value == 0 {
                combination_iter += 1;
            }
        }

        cache.insert(cache_key, combination_iter);

        combination += combination_iter;
    };

    for line in input.lines() {
        let (turn, num) = line.split_at(1);
        let num: i64 = num.parse().expect("failed to parse number");

        match turn {
            "R" => rotate(num),
            "L" => rotate(-num),
            _ => panic!("invalid turn"),
        };
    }

    println!("{}", combination);

    Ok(())
}
