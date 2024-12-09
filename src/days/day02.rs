use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day02").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let num_safe_reports = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter_map(|nums| {
            (nums
                .iter()
                .skip(1)
                .zip(nums.iter())
                .all(|(num1, num2)| (num1 < num2) && (num1.abs_diff(*num2) <= 3))
                || nums
                    .iter()
                    .skip(1)
                    .zip(nums.iter())
                    .all(|(num1, num2)| (num1 > num2) && (num1.abs_diff(*num2) <= 3)))
            .then_some(1)
        })
        .count();

    println!("Safe reports: {num_safe_reports}");
    let num_safe_reports_dampener = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter_map(|nums| {
            if let Some(first_val) = nums.iter().next() {
                let mut dampener = true;
                let mut check_other_dir = false;
                let mut last_val = first_val;
                for i in nums.iter().skip(1) {
                    if (last_val < i) && (last_val.abs_diff(*i) <= 3) {
                        last_val = i;
                    } else if dampener {
                        dampener = false;
                        continue;
                    } else {
                        check_other_dir = true;
                        break;
                    }
                }

                if !check_other_dir {
                    return Some(nums);
                }

                check_other_dir = false;
                last_val = nums.iter().skip(1).next().unwrap_or(&0);
                for i in nums.iter().skip(2) {
                    if (last_val < i) && (last_val.abs_diff(*i) <= 3) {
                        last_val = i;
                    } else {
                        check_other_dir = true;
                        break;
                    }
                }

                if !check_other_dir {
                    return Some(nums);
                }

                dampener = true;
                last_val = first_val;
                check_other_dir = false;
                for i in nums.iter().skip(1) {
                    if (i < last_val) && (last_val.abs_diff(*i) <= 3) {
                        last_val = i;
                    } else if dampener {
                        dampener = false;
                        continue;
                    } else {
                        check_other_dir = true;
                        break;
                    }
                }

                if !check_other_dir {
                    return Some(nums);
                }

                last_val = nums.iter().skip(1).next().unwrap_or(&0);
                for i in nums.iter().skip(2) {
                    if (i < last_val) && (last_val.abs_diff(*i) <= 3) {
                        last_val = i;
                    } else {
                        return None;
                    }
                }

                return Some(nums);
            }
            Some(nums)
        })
        //.inspect(|nums| println!("{:?}", nums))
        .count();

    println!("Safe reports dampener: {num_safe_reports_dampener}");
}
