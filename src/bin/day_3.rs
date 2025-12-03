use advent_of_code_2025::fetch_input;

// after brute force tried llm, nice solution actually
fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(3, 2025).expect("failed to fetch input");

    let mut sum: u128 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u8> = line.chars().map(|c| c as u8 - b'0').collect();
        let n = digits.len();
        let k = 12usize;

        if k == 0 || n == 0 || k > n {
            continue;
        }

        let mut result: u128 = 0;
        let mut start = 0usize;

        for i in 0..k {
            let end_inclusive = n - (k - i);

            let mut best_digit = 0u8;
            let mut best_idx = start;

            for idx in start..=end_inclusive {
                let d = digits[idx];
                if d > best_digit {
                    best_digit = d;
                    best_idx = idx;
                    if best_digit == 9 {
                        break;
                    }
                }
            }

            result = result * 10 + best_digit as u128;
            start = best_idx + 1;
        }

        sum += result;
    }

    println!("{}", sum);

    Ok(())
}
