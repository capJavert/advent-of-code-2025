use advent_of_code_2025::fetch_input;

#[derive(Debug)]
struct MathProblem {
    values: Vec<u64>,
    operator: char,
}

impl MathProblem {
    fn new() -> Self {
        MathProblem {
            values: Vec::new(),
            operator: 'u',
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(6, 2025).expect("failed to fetch input");

    // Parse input as a grid of characters so we can read columns.
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    if lines.is_empty() {
        println!("0");
        return Ok(());
    }

    // Ensure all lines have the same width by padding with spaces.
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for line in &mut lines {
        if line.len() < width {
            line.push_str(&" ".repeat(width - line.len()));
        }
    }

    // Build columns and find separator columns (all spaces).
    let rows = lines.len();
    let mut is_separator = vec![false; width];
    for c in 0..width {
        let mut all_space = true;
        for r in 0..rows {
            if lines[r].as_bytes()[c] != b' ' {
                all_space = false;
                break;
            }
        }
        is_separator[c] = all_space;
    }

    // Group consecutive non-separator columns into problems (left-to-right),
    // then we'll read the groups right-to-left as the cephalopods do.
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut cur: Option<Vec<usize>> = None;
    for c in 0..width {
        if is_separator[c] {
            if let Some(g) = cur.take() {
                groups.push(g);
            }
        } else {
            if cur.is_none() {
                cur = Some(Vec::new());
            }
            cur.as_mut().unwrap().push(c);
        }
    }
    if let Some(g) = cur.take() {
        groups.push(g);
    }

    // We'll produce math_problems by reading groups from right-to-left.
    let mut math_problems: Vec<MathProblem> = Vec::new();
    for group in groups.into_iter().rev() {
        if group.is_empty() {
            continue;
        }

        let mut mp = MathProblem::new();

        // For each column in the group (left-to-right within the group),
        // build the number by reading rows from top to the row before the last
        // (the bottom row contains the operator for the problem).
        for &col in group.iter().rev() {
            let mut digits = String::new();
            for r in 0..rows - 1 {
                let ch = lines[r].as_bytes()[col] as char;
                if ch != ' ' {
                    digits.push(ch);
                }
            }
            if !digits.is_empty() {
                if let Ok(value) = digits.parse::<u64>() {
                    mp.values.push(value);
                }
            }
        }

        // Operator is taken from the bottom row at the non-space column inside the group.
        let mut op_char = 'u';
        for &col in &group {
            let ch = lines[rows - 1].as_bytes()[col] as char;
            if ch != ' ' {
                op_char = ch;
                break;
            }
        }
        if op_char == 'u' {
            panic!("no operator found for group");
        }
        mp.operator = op_char;

        math_problems.push(mp);
    }

    let mut sum = 0;

    for math_problem in math_problems {
        let result = match math_problem.operator {
            '+' => math_problem.values.iter().sum::<u64>(),
            '*' => math_problem.values.iter().product::<u64>(),
            _ => panic!("invalid operator"),
        };

        sum += result;
    }

    println!("{}", sum);

    Ok(())
}
