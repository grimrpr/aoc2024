use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day03").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let line_joined = lines.join("");

    let mul_sum: i32 = line_joined
        .match_indices("mul(")
        .filter_map(|(idx, _)| match line_joined[idx..].find(")") {
            Some(closing_idx) if closing_idx < 12 => {
                line_joined[(idx + 4)..(idx + closing_idx)].split_once(",")
            }
            _ => None,
        })
        .filter_map(|(op_left, op_right)| {
            match (op_left.parse::<i32>().ok(), op_right.parse::<i32>().ok()) {
                (Some(num_left), Some(num_right)) => Some(num_left * num_right),
                _ => None,
            }
        })
        .sum::<i32>();
    println!("Sum of products: {mul_sum}");

    let mul_sum_enabled: i32 = mul_sum
        - line_joined
            .match_indices("don't()")
            .map(|(idx, _)| {
                match (
                    line_joined[(idx + 7)..].find("don't()"),
                    line_joined[idx..].find("do()"),
                ) {
                    (Some(end_idx_dont), Some(end_idx_do)) => {
                        &line_joined[idx..(idx + std::cmp::min(end_idx_do, end_idx_dont + 7))]
                    }
                    (None, Some(end_idx_do)) => &line_joined[idx..(idx + end_idx_do)],
                    (Some(end_idx_dont), None) => &line_joined[idx..(idx + end_idx_dont + 7)],
                    _ => &line_joined[idx..],
                }
            })
            //.inspect(|dont_str| println!("{dont_str}\n"))
            .map(|line| {
                line.match_indices("mul(")
                    .filter_map(|(idx, _)| match line[idx..].find(")") {
                        Some(closing_idx) if closing_idx < 12 => {
                            line[(idx + 4)..(idx + closing_idx)].split_once(",")
                        }
                        _ => None,
                    })
                    .filter_map(|(op_left, op_right)| {
                        match (op_left.parse::<i32>().ok(), op_right.parse::<i32>().ok()) {
                            (Some(num_left), Some(num_right)) => Some(num_left * num_right),
                            _ => None,
                        }
                    })
                    .sum::<i32>()
            })
            .sum::<i32>();

    println!("Sum of products: {mul_sum_enabled}");
}
