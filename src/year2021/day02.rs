use crate::input::{InputData, ParseStream, ParseYolo};

enum Direction {
    Forward,
    Down,
    Up,
}

impl ParseYolo<'_> for Direction {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        if stream.try_consume("forward") {
            Self::Forward
        } else if stream.try_consume("down") {
            Self::Down
        } else if stream.try_consume("up") {
            Self::Up
        } else {
            panic!("Unexpected direction")
        }
    }
}

struct Instruction {
    direction: Direction,
    amount: i64,
}

impl ParseYolo<'_> for Instruction {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        let direction = stream.parse_yolo();
        stream.expect(" ");
        let amount = stream.parse_yolo();
        Self { direction, amount }
    }
}

pub fn part_1(input: &InputData) -> i64 {
    let (final_horizontal, final_depth) = input.lines_as::<Instruction>()
        .fold((0, 0), |(horizontal, depth), Instruction { direction, amount }| {
            match direction {
                Direction::Forward => (horizontal + amount, depth),
                Direction::Down => (horizontal, depth + amount),
                Direction::Up => (horizontal, depth - amount),
            }
        });

    final_horizontal * final_depth
}

pub fn part_2(input: &InputData) -> i64 {
    let (final_horizontal, final_depth, _) = input.lines_as::<Instruction>()
        .fold((0, 0, 0), |(horizontal, depth, aim), Instruction { direction, amount }| {
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
    use crate::input::InputData;

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

    fn data() -> InputData {
        InputData::from_string("
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ")
    }
}