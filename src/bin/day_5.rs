use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(5, 2025).expect("failed to fetch input");

    let [ranges_input, _ids_input] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut ranges = ranges_input
        .lines()
        .map(|line| {
            let [start, end] = line.split('-').collect::<Vec<&str>>().try_into().unwrap();

            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let mut fresh_ingredients_count = 0;

    // check ranges for overlap, create new ranges list without overlaps
    // Approach: sort ranges by start, then merge overlapping or contiguous ranges
    ranges.sort_by_key(|r| r.0);

    let mut nonoverlap_ranges: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if let Some((_last_start, last_end)) = nonoverlap_ranges.last_mut() {
            // overlap or contiguous if start <= last_end (contiguous considered merged)
            if start <= *last_end {
                // extend the last range's end if needed
                if end > *last_end {
                    *last_end = end;
                }
            } else {
                nonoverlap_ranges.push((start, end));
            }
        } else {
            nonoverlap_ranges.push((start, end));
        }
    }

    for (start, end) in nonoverlap_ranges {
        fresh_ingredients_count += end - start + 1;
    }

    println!("{}", fresh_ingredients_count);

    Ok(())
}
