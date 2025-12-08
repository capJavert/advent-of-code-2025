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

    let mut math_problems = vec![];

    let lines_count = input.lines().count();

    for (line_index, line) in input.lines().enumerate() {
        for (problem_index, part) in line.split_whitespace().enumerate() {
            if math_problems.get(problem_index).is_none() {
                math_problems.push(MathProblem::new());
            }

            let math_problem = math_problems.get_mut(problem_index).unwrap();

            if line_index < lines_count - 1 {
                if let Ok(value) = part.parse::<u64>() {
                    math_problem.values.push(value);
                }
            } else {
                if let Some(operator) = part.chars().next() {
                    math_problem.operator = operator;
                }
            }
        }
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
