#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Start,
    Empty,
    Splitter,
    Beam(usize),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '^' => Self::Splitter,
            '.' => Self::Empty,
            '|' => Self::Beam(0),
            _ => unreachable!(),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "S     "),
            Self::Empty => write!(f, ".     "),
            Self::Splitter => write!(f, "^     "),
            Self::Beam(n) => write!(f, "|({n:3})"),
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let space = reader
            .lines()
            .map(|l| l.unwrap().chars().map(Tile::from).collect_vec())
            .collect_vec();

        let (counter, _) =
            space
                .into_iter()
                .fold((0, Vec::new()), |(mut counter, mut acc), next| {
                    if acc.is_empty() {
                        acc.push(next);
                        return (counter, acc);
                    }

                    let prev = acc.pop().unwrap();
                    let mut changes = vec![];
                    let len = next.len();
                    let mut next = next
                        .into_iter()
                        .enumerate()
                        .map(|(index, tile)| match tile {
                            Tile::Empty => prev.get(index).map_or(tile, |top| match top {
                                Tile::Start => Tile::Beam(0),
                                Tile::Beam(n) => Tile::Beam(*n),
                                _ => Tile::Empty,
                            }),
                            Tile::Splitter => prev.get(index).map_or(tile, |top| match top {
                                Tile::Beam(n) => {
                                    if index > 0 && index < len {
                                        changes.push((index - 1, Tile::Beam(*n)));
                                        changes.push((index + 1, Tile::Beam(*n)));
                                    }
                                    counter += 1;

                                    Tile::Splitter
                                }
                                _ => Tile::Splitter,
                            }),

                            _ => tile,
                        })
                        .collect_vec();

                    while let Some((index, new_tile)) = changes.pop() {
                        if let Some(tile) = next.get_mut(index) {
                            *tile = new_tile;
                        }
                    }

                    acc.push(prev);
                    acc.push(next);
                    (counter, acc)
                });

        Ok(counter)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let space = reader
            .lines()
            .map(|l| l.unwrap().chars().map(Tile::from).collect_vec())
            .collect_vec();

        let acc = space.into_iter().fold(Vec::new(), |mut acc, next| {
            if acc.is_empty() {
                acc.push(next);
                return acc;
            }

            let prev = acc.pop().unwrap();
            let mut changes = vec![];
            let len = next.len();
            let mut next = next
                .into_iter()
                .enumerate()
                .map(|(index, tile)| match tile {
                    Tile::Empty => prev.get(index).map_or(tile, |top| match top {
                        Tile::Start => Tile::Beam(1),
                        Tile::Beam(n) => Tile::Beam(*n),
                        _ => Tile::Empty,
                    }),
                    Tile::Splitter => prev.get(index).map_or(tile, |top| match top {
                        Tile::Beam(n) => {
                            if index > 0 {
                                changes.push((index - 1, Tile::Beam(*n)));
                            }

                            if index < len {
                                changes.push((index + 1, Tile::Beam(*n)));
                            }

                            Tile::Splitter
                        }
                        _ => Tile::Splitter,
                    }),

                    _ => tile,
                })
                .collect_vec();

            for (index, new_tile) in changes.into_iter().rev().collect_vec() {
                if let Some(tile) = next.get_mut(index) {
                    let new_tile_ = match (&tile, new_tile.clone()) {
                        (Tile::Beam(n), Tile::Beam(m)) => Tile::Beam(n + m),
                        (_, tile) => tile,
                    };

                    *tile = new_tile_;
                }
            }

            acc.push(prev);
            acc.push(next);
            acc
        });

        let result = acc
            .last()
            .unwrap()
            .iter()
            .map(|tile| match tile {
                Tile::Beam(n) => *n,
                _ => 0,
            })
            .sum::<usize>();

        Ok(result)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
