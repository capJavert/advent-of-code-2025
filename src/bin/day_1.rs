use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(1, 2025).expect("failed to fetch input");

    let mut value = 50;

    let rotate = |current, num| -> i32 { ((current + num) % 100 + 100) % 100 };

    let mut combination = 0;

    for line in input.lines() {
        let (turn, num) = line.split_at(1);
        let num: i32 = num.parse().expect("failed to parse number");

        value = match turn {
            "R" => rotate(value, num),
            "L" => rotate(value, -num),
            _ => panic!("invalid turn"),
        };

        if value == 0 {
            combination += 1;
        }
    }

    println!("{}", combination);

    Ok(())
}
