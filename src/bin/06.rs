#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (nums, ops) = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .split(" ")
                    .filter(|l| !l.is_empty())
                    .map(|l| l.to_string())
                    .collect_vec()
            })
            .fold((vec![], vec![]), |(mut nums, mut ops), s| {
                let new_nums = s
                    .iter()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_vec();

                if new_nums.is_empty() {
                    ops.push(s);
                } else {
                    nums.push(new_nums);
                }

                (nums, ops)
            });

        let ops = ops.first().unwrap();

        let result = ops
            .iter()
            .enumerate()
            .map(|(i, op)| {
                let is_mul = op == "*";
                let start = if is_mul { 1 } else { 0 };
                nums.iter()
                    .map(|n| n.get(i).unwrap())
                    .fold(start, |acc, n| if is_mul { acc * n } else { acc + n })
            })
            .sum::<usize>();

        Ok(result)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map(|l| l.unwrap()).collect_vec();

        let ops = lines.pop().unwrap();

        let ops = ops.chars().enumerate().fold(
            Vec::<(char, Vec<usize>)>::new(),
            |mut result, (i, op)| {
                let nums = lines
                    .iter()
                    .map(|line| line.chars().nth(i).unwrap())
                    .filter(|char| *char != ' ')
                    .join("");

                if nums.is_empty() {
                    return result;
                }

                match op {
                    '*' | '+' => result.push((op, vec![])),
                    _ => (),
                };

                let last = result.last_mut().unwrap();
                last.1.push(nums.parse::<usize>().unwrap());

                result
            },
        );

        let result = ops
            .into_iter()
            .map(|(op, nums)| match op {
                '*' => nums.iter().product::<usize>(),
                '+' => nums.iter().sum(),
                _ => todo!(),
            })
            .sum::<usize>();

        Ok(result)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
