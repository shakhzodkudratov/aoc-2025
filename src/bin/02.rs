#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn magic_number(n: usize) -> usize {
    (0..n - 1).fold(1, |i, _| i * 10 + 1)
}

fn solver<R: BufRead>(reader: R, only_twice: bool) -> Result<usize> {
    let line = reader
        .lines()
        .next()
        .ok_or(anyhow!("failed to read the line"))??;

    let result = line
        .split(',')
        .map(|r| {
            r.split('-')
                .map(|r| r.parse().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .map(|(start, end)| start..=end)
        .flat_map(|range| {
            range
                .filter(|num| {
                    if *num < 10 {
                        return false;
                    }

                    let digits = num.to_string().len();
                    let mn = magic_number(digits);

                    if !digits.is_multiple_of(2) && only_twice {
                        return false;
                    };

                    let groups = if only_twice {
                        let mut groups = (1..digits)
                            .filter(|d| digits % d == 0)
                            .rev()
                            .take(1)
                            .collect_vec();
                        groups.push(1);
                        groups
                    } else {
                        (1..digits).filter(|d| digits % d == 0).collect_vec()
                    };

                    groups
                        .into_iter()
                        .map(magic_number)
                        .map(|d| mn / d)
                        .any(|d| num % d == 0)
                })
                .collect_vec()
        })
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        solver(reader, true)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        solver(reader, false)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
