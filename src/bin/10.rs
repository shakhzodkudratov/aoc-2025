#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[derive(Default, Debug, Clone)]
struct Manual {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Manual {
    fn apply(&mut self, parser: Parser) {
        match parser {
            Parser::Lights(items) => items.into_iter().for_each(|item| self.lights.push(item)),
            Parser::Buttons(items) => self.buttons.push(items),
            Parser::Joltages(items) => items.into_iter().for_each(|item| self.joltages.push(item)),
        };
    }
}

#[derive(Debug)]
enum Parser {
    Lights(Vec<bool>),
    Buttons(Vec<usize>),
    Joltages(Vec<usize>),
}

impl TryFrom<&str> for Parser {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut value = value.chars().collect::<VecDeque<_>>();
        let start = value.pop_front().ok_or(anyhow!("failed to parse"))?;
        let end = value.pop_back().ok_or(anyhow!("failed to parse"))?;

        match (start, end) {
            ('[', ']') => Ok(Parser::Lights(
                value.iter().map(|light| *light == '#').collect(),
            )),
            ('(', ')') => Ok(Parser::Buttons(
                value
                    .iter()
                    .join("")
                    .split(",")
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect(),
            )),
            ('{', '}') => Ok(Parser::Joltages(
                value
                    .iter()
                    .join("")
                    .split(",")
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect(),
            )),
            _ => Err(anyhow!("unknown pattern")),
        }
    }
}

fn test_buttons(input: &[bool], buttons: &[&Vec<usize>]) -> bool {
    let initial = (0..input.len()).map(|_| false).collect_vec();

    buttons
        .iter()
        .fold(initial, |acc, buttons| {
            buttons.iter().fold(acc, |mut acc, button| {
                let lamp = acc.get_mut(*button).unwrap();
                *lamp = !*lamp;
                acc
            })
        })
        .eq(input)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let manuals = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                line.split(" ")
                    .filter_map(|pattern| Parser::try_from(pattern).ok())
                    .fold(Manual::default(), |mut acc, parser| {
                        acc.apply(parser);
                        acc
                    })
            })
            .collect_vec();

        let answer = manuals
            .iter()
            .filter_map(|manual| {
                manual
                    .buttons
                    .iter()
                    .powerset()
                    .filter(|actions| test_buttons(&manual.lights, actions))
                    .map(|actions| actions.len())
                    .min()
            })
            .sum::<usize>();

        Ok(answer)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
