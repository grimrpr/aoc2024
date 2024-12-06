use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day01").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let (mut left_num, mut right_num): (Vec<usize>, Vec<usize>) = lines
        .iter()
        //.inspect(|line| println!("{line}"))
        .filter_map(|line| line.split_once(char::is_whitespace))
        .filter_map(|(str_num_left, str_num_right)| {
            match (
                str_num_left.trim().parse::<usize>().ok(),
                str_num_right.trim().parse::<usize>().ok(),
            ) {
                (Some(left_num), Some(right_num)) => Some((left_num, right_num)),
                _ => None,
            }
        })
        .unzip();

    left_num.sort();
    right_num.sort();

    let diff_result = left_num
        .iter()
        .zip(right_num.clone())
        .map(|(left_num, right_num)| left_num.abs_diff(right_num))
        .sum::<usize>();

    println!("Difference result: {diff_result}");

    let right_num_count = right_num.iter().fold(HashMap::new(), |mut acc_map, num| {
        acc_map
            .entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        acc_map
    });

    let similarity_sum: usize = left_num
        .iter()
        .map(|num| num * right_num_count.get(num).unwrap_or(&0))
        .sum();

    println!("Similarity result: {similarity_sum}");
}
