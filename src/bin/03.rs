#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

fn solver<R: BufRead>(reader: R, times: usize) -> Result<usize> {
    let answer = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split("")
                .filter_map(|d| d.parse::<usize>().ok())
                .enumerate()
                .collect_vec()
        })
        .map(|nums| {
            let mut result: Vec<usize> = vec![];

            let mut left = 0;
            let mut right = nums.len() - times + 1;

            while result.len() < times && left < nums.len() {
                let window = &nums[left..right];

                let max = window.iter().max_set_by(|a, b| a.1.cmp(&b.1));
                let (index, num) = max.first().unwrap();

                result.push(*num);

                left = index + 1;
                if right < nums.len() {
                    right += 1;
                }
            }

            result
                .into_iter()
                .reduce(|acc, d| acc * 10 + d)
                .unwrap_or(0)
        })
        .sum();

    Ok(answer)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        solver(reader, 2)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        solver(reader, 12)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
