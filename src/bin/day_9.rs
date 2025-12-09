use advent_of_code_2025::fetch_input;

fn get_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();

    (dx + 1) * (dy + 1)
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(9, 2025).expect("failed to fetch input");

    let points = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                Some((
                    parts[0].trim().parse::<i64>().ok()?,
                    parts[1].trim().parse::<i64>().ok()?,
                ))
            } else {
                panic!("Invalid line format: {}", line);
            }
        })
        .collect::<Vec<(i64, i64)>>();

    let mut largest_area = 0;

    for a in points.iter() {
        for b in points.iter() {
            if a == b {
                continue;
            }

            let area = get_area(*a, *b);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("{}", largest_area);

    Ok(())
}
