use std::str::FromStr;

use crate::input::{InputData, IteratorYoloParsing, ParseYolo};

enum Direction {
    Forward,
    Down,
    Up,
}

impl ParseYolo for Direction {
    fn parse(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => panic!("Unexpected direction"),
        }
    }
}

struct Command {
    direction: Direction,
    amount: i64,
}

impl ParseYolo for Command {
    fn parse(s: &str) -> Self {
        let (direction, amount) = s.split_once(" ").unwrap();
        Self {
            direction: Direction::parse(direction),
            amount: amount.parse::<i64>().unwrap(),
        }
    }
}

pub fn part_1<I: InputData>(input: &I) -> i64 {
    let (final_horizontal, final_depth) = input.lines()
        .parse_yolo::<Command>()
        .fold((0, 0), |(horizontal, depth), Command { direction, amount }| {
            match direction {
                Direction::Forward => (horizontal + amount, depth),
                Direction::Down => (horizontal, depth + amount),
                Direction::Up => (horizontal, depth - amount),
            }
        });

    final_horizontal * final_depth
}

pub fn part_2<I: InputData>(input: &I) -> i64 {
    let (final_horizontal, final_depth, _) = input.lines()
        .parse_yolo::<Command>()
        .fold((0, 0, 0), |(horizontal, depth, aim), Command { direction, amount }| {
            match direction {
                Direction::Forward => (horizontal + amount, depth + aim * amount, aim),
                Direction::Down => (horizontal, depth, aim + amount),
                Direction::Up => (horizontal, depth, aim - amount),
            }
        });

    final_horizontal * final_depth
}


#[cfg(test)]
mod tests {
    use crate::input::StringInputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 150);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 900);
    }

    fn data() -> StringInputData {
        StringInputData::new("
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ")
    }
}