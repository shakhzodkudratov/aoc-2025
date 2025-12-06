#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (_, range, ids) = reader.lines().map(|line| line.unwrap()).fold(
            (true, vec![], vec![]),
            |(is_range, mut range, mut ids), l| {
                if l.is_empty() {
                    (false, range, ids)
                } else if is_range {
                    range.push(l);
                    (is_range, range, ids)
                } else {
                    ids.push(l);
                    (is_range, range, ids)
                }
            },
        );
        let range = range
            .into_iter()
            .map(|line| {
                // let (start, end) =

                line.split("-")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize)>()
                    .unwrap()

                // start..=end
            })
            .collect_vec();

        let ids = ids
            .into_iter()
            .map(|l| l.parse::<usize>().unwrap())
            .collect_vec();

        let result = ids
            .iter()
            .filter(|id| range.iter().any(|(start, end)| id >= &start && id <= &end))
            .count();

        Ok(result)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (_, range, _) = reader.lines().map(|line| line.unwrap()).fold(
            (true, vec![], vec![]),
            |(is_range, mut range, mut ids), l| {
                if l.is_empty() {
                    (false, range, ids)
                } else if is_range {
                    range.push(l);
                    (is_range, range, ids)
                } else {
                    ids.push(l);
                    (is_range, range, ids)
                }
            },
        );
        let range = range
            .into_iter()
            .map(|line| {
                line.split("-")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize)>()
                    .unwrap()
            })
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .fold(Vec::<(usize, usize)>::new(), |mut acc, current| {
                match acc
                    .iter_mut()
                    // .find(|last| last.0 <= current.0 && last.1 < current.1 && last.1 >= current.0)
                    .find(|last| last.0 <= current.0 && last.1 >= current.0 - 1)
                {
                    Some(last) => {
                        last.0 = current.0.min(last.0);
                        last.1 = current.1.max(last.1);
                    }
                    None => {
                        acc.push(current);
                    }
                };

                acc
            });

        // println!("{range:?}");

        let result = range
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<usize>();

        Ok(result)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
