use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
L200";

const BOBOSHER_TEST: &str = "\
L60
R110
R200
L1";

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct RotationResult {
    pub new_position: u32,
    pub zero_crosses: u32,
}

impl Rotation {
    fn rotate(&self, dial: u32) -> RotationResult {
        let num = match self {
            Rotation::Left(num) => num,
            Rotation::Right(num) => num,
        };

        if *num == 0 {
            return RotationResult {
                new_position: dial,
                zero_crosses: 0,
            };
        }

        let rotations = (num - (num % 100)) / 100;
        let zero_crosses = match self {
            Rotation::Left(_) => {
                if dial != 0 && dial <= *num % 100 {
                    1
                } else {
                    0
                }
            }
            Rotation::Right(_) => {
                if (100 - dial) <= *num % 100 {
                    1
                } else {
                    0
                }
            }
        };
        let zero_crosses = zero_crosses + rotations;

        let new_position = match self {
            Rotation::Left(_) => {
                let normalizer = ((num - (num % 100) % 100) + 1) * 100;
                let result = (dial as i32) - (*num as i32) + (normalizer as i32);
                if result.is_negative() {
                    -result
                } else {
                    result
                }
            }
            Rotation::Right(_) => (dial + num) as i32,
        };
        let new_position = (new_position % 100) as u32;
        let new_position = if new_position == 100 { 0 } else { new_position };

        RotationResult {
            new_position,
            zero_crosses,
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let actions = reader.lines();

        // dial position, 0 count
        let (_, result) =
            parse_actions(actions)
                .into_iter()
                .fold((50, 0), |(dial_position, result), action| {
                    let RotationResult {
                        new_position,
                        zero_crosses: _zero_crosses,
                    } = action.rotate(dial_position);

                    (new_position, result + usize::from(new_position == 0))
                });
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let actions = reader.lines();

        // dial position, 0 count
        let (_, result) = parse_actions(actions).into_iter().enumerate().fold(
            (50, 0usize),
            |(dial_position, result), (i, action)| {
                let RotationResult {
                    new_position,
                    zero_crosses,
                } = action.rotate(dial_position);

                (new_position, result + (zero_crosses as usize))
            },
        );

        Ok(result)
    }

    assert_eq!(8, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(5, part2(BufReader::new(BOBOSHER_TEST.as_bytes()))?);
    assert_eq!(52, part2(BufReader::new(BOBOSHER_TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_actions<R: BufRead>(actions: io::Lines<R>) -> Vec<Rotation> {
    actions
        .into_iter()
        .map(|l| {
            let l = l.unwrap();
            let num = l[1..].parse::<u32>().unwrap();
            if l.starts_with('L') {
                Rotation::Left(num)
            } else {
                Rotation::Right(num)
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotation() {
        assert_eq!(
            Rotation::Left(50).rotate(50),
            RotationResult {
                new_position: 0,
                zero_crosses: 1,
            }
        );
        assert_eq!(
            Rotation::Left(50).rotate(51),
            RotationResult {
                new_position: 1,
                zero_crosses: 0,
            }
        );
        assert_eq!(
            Rotation::Left(50).rotate(49),
            RotationResult {
                new_position: 99,
                zero_crosses: 1,
            }
        );
        assert_eq!(
            Rotation::Left(100).rotate(50),
            RotationResult {
                new_position: 50,
                zero_crosses: 1,
            }
        );
        assert_eq!(
            Rotation::Left(101).rotate(50),
            RotationResult {
                new_position: 49,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Left(99).rotate(50),
            RotationResult {
                new_position: 51,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Left(200).rotate(50),
            RotationResult {
                new_position: 50,
                zero_crosses: 2
            }
        );
        assert_eq!(
            Rotation::Left(2000).rotate(50),
            RotationResult {
                new_position: 50,
                zero_crosses: 20,
            }
        );
        assert_eq!(
            Rotation::Left(115).rotate(15),
            RotationResult {
                new_position: 0,
                zero_crosses: 2
            }
        );
        assert_eq!(
            Rotation::Left(1).rotate(0),
            RotationResult {
                new_position: 99,
                zero_crosses: 0
            }
        );
        assert_eq!(
            Rotation::Left(612).rotate(81),
            RotationResult {
                new_position: 69,
                zero_crosses: 6
            }
        );
    }

    #[test]
    fn test_right_rotation() {
        assert_eq!(
            Rotation::Right(50).rotate(0),
            RotationResult {
                new_position: 50,
                zero_crosses: 0
            }
        );
        assert_eq!(
            Rotation::Right(50).rotate(50),
            RotationResult {
                new_position: 0,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Right(50).rotate(51),
            RotationResult {
                new_position: 1,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Right(50).rotate(49),
            RotationResult {
                new_position: 99,
                zero_crosses: 0
            }
        );
        assert_eq!(
            Rotation::Right(100).rotate(50),
            RotationResult {
                new_position: 50,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Right(101).rotate(50),
            RotationResult {
                new_position: 51,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Right(99).rotate(50),
            RotationResult {
                new_position: 49,
                zero_crosses: 1
            }
        );
        assert_eq!(
            Rotation::Right(200).rotate(50),
            RotationResult {
                new_position: 50,
                zero_crosses: 2
            }
        );
        assert_eq!(
            Rotation::Right(2000).rotate(50),
            RotationResult {
                new_position: 50,
                zero_crosses: 20,
            }
        );
        assert_eq!(
            Rotation::Right(110).rotate(90),
            RotationResult {
                new_position: 0,
                zero_crosses: 2
            }
        );
    }
}
