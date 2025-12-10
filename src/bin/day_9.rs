use advent_of_code_2025::fetch_input;
use std::collections::HashMap;

fn get_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();

    (dx + 1) * (dy + 1)
}

fn compress_coordinates(
    points: &Vec<(i64, i64)>,
) -> (Vec<i64>, Vec<i64>, HashMap<i64, usize>, HashMap<i64, usize>) {
    let mut x_coords: Vec<i64> = points.iter().map(|(x, _)| *x).collect();
    let mut y_coords: Vec<i64> = points.iter().map(|(_, y)| *y).collect();

    x_coords.sort_unstable();
    x_coords.dedup();
    y_coords.sort_unstable();
    y_coords.dedup();

    let x_map: HashMap<i64, usize> = x_coords.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map: HashMap<i64, usize> = y_coords.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    (x_coords, y_coords, x_map, y_map)
}

fn point_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64, eps: f64) -> bool {
    let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
    if (cross as f64).abs() > eps {
        return false;
    }

    let dot = (px - x1) * (x2 - x1) + (py - y1) * (y2 - y1);
    if dot < 0 {
        return false;
    }

    let sq_len = (x2 - x1).pow(2) + (y2 - y1).pow(2);
    if dot > sq_len {
        return false;
    }

    true
}

fn point_in_polygon(x: i64, y: i64, poly: &Vec<(i64, i64)>) -> bool {
    let mut inside = false;
    let n = poly.len();

    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % n];

        if point_on_segment(x, y, x1, y1, x2, y2, 1e-9) {
            return true;
        }

        if (y1 > y) != (y2 > y) {
            let x_intersect = (x2 - x1) * (y - y1) / (y2 - y1) + x1;
            if x < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
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

    let (x_coords, y_coords, x_map, y_map) = compress_coordinates(&points);

    let compressed_points: Vec<(usize, usize)> = points
        .iter()
        .map(|(x, y)| (*x_map.get(x).unwrap(), *y_map.get(y).unwrap()))
        .collect();

    let mut largest_area = 0;

    for (_index_a, &a_compressed) in compressed_points.iter().enumerate() {
        let a = (x_coords[a_compressed.0], y_coords[a_compressed.1]);

        'checker: for (_index_b, &b_compressed) in compressed_points.iter().enumerate() {
            if a_compressed == b_compressed {
                continue;
            }

            let b = (x_coords[b_compressed.0], y_coords[b_compressed.1]);

            let area = get_area(a, b);

            if area > largest_area {
                let min_x_idx = a_compressed.0.min(b_compressed.0);
                let max_x_idx = a_compressed.0.max(b_compressed.0);
                let min_y_idx = a_compressed.1.min(b_compressed.1);
                let max_y_idx = a_compressed.1.max(b_compressed.1);

                for x_idx in min_x_idx..=max_x_idx {
                    for y_idx in min_y_idx..=max_y_idx {
                        let x = x_coords[x_idx];
                        let y = y_coords[y_idx];
                        let is_inside = point_in_polygon(x, y, &points);

                        if !is_inside {
                            continue 'checker;
                        }
                    }
                }

                largest_area = area;
            }
        }
    }

    println!("{}", largest_area);

    Ok(())
}
