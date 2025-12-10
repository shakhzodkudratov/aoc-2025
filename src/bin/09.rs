#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let nums = reader
            .lines()
            .filter_map(|line| {
                line.unwrap()
                    .split(",")
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect_tuple::<(usize, usize)>()
            })
            .collect_vec();

        let answer = nums
            .iter()
            .flat_map(|num1| {
                nums.iter()
                    .map(|num2| {
                        let a = num1.0.abs_diff(num2.0) + 1;
                        let b = num1.1.abs_diff(num2.1) + 1;
                        a * b
                    })
                    .collect_vec()
            })
            .sorted_by(|a, b| b.cmp(a))
            .take(1)
            .collect_vec();

        let answer = answer.first().unwrap();

        println!("{answer:?}");

        Ok(*answer)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let nums = reader
            .lines()
            .filter_map(|line| {
                line.unwrap()
                    .split(",")
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect_tuple::<(usize, usize)>()
            })
            .collect_vec();

        let borders = nums
            .iter()
            .flat_map(|num1| {
                nums.iter()
                    .filter(|num2| !num2.eq(&num1))
                    .filter(|num2| num1.0 == num2.0 || num1.1 == num2.1)
                    .map(|num2| {
                        if num1.0 < num2.0 || num1.1 < num2.1 {
                            (num1, num2)
                        } else {
                            (num2, num1)
                        }
                    })
                    .collect_vec()
            })
            .sorted_by(|a, b| b.cmp(a))
            .dedup()
            .collect_vec();

        println!("{borders:?}");

        // let answer = answer.first().unwrap();

        // println!("{answer:?}");

        // Ok(*answer)
        Ok(0)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
