use ahash::HashSet;
use bstr::ByteSlice;
use itertools::Itertools;

use crate::array::Coordinate2d;
use crate::input::{InputData, ParseStream, ParseYolo};

pub fn part_1(input: &InputData) -> usize {
    let (dots, instructions) = input.raw().split_once_str("\n\n").unwrap();
    let instructions = ParseStream::new(instructions).parse_iter::<Fold>("\n").collect_vec();
    let first_instruction = &instructions[0];

    let unique_dots: HashSet<_> = ParseStream::new(dots).parse_iter::<Dot>("\n")
        .map(|dot| first_instruction.transform(dot.to_coordinate()))
        .collect();

    unique_dots.len()
}

pub fn part_2(input: &InputData) -> String {
    let (dots, instructions) = input.raw().split_once_str("\n\n").unwrap();
    let instructions = ParseStream::new(instructions).parse_iter::<Fold>("\n").collect_vec();
    let num_rows = instructions.iter()
        .copied()
        .filter_map(|fold| if let Fold::Up(y) = fold { Some(y) } else { None })
        .min()
        .unwrap();
    let num_columns = instructions.iter()
        .copied()
        .filter_map(|fold| if let Fold::Left(x) = fold { Some(x) } else { None })
        .min()
        .unwrap() + 1;

    let mut result = vec!['.'; (num_rows * num_columns) as usize];
    for i in 0..num_rows {
        result[((i + 1) * num_columns - 1) as usize] = '\n';
    }
    ParseStream::new(dots).parse_iter::<Dot>("\n")
        .map(|dot| instructions.iter().fold(dot.to_coordinate(), |dot, instruction| instruction.transform(dot)))
        .for_each(|dot| result[(dot.row() * num_columns + dot.column()) as usize] = '#');

    String::from_iter(result)
}


#[derive(Copy, Clone)]
enum Fold {
    Up(isize),
    Left(isize),
}

impl Fold {
    fn transform(&self, point: Coordinate2d) -> Coordinate2d {
        match self {
            Fold::Up(y) => if point.row() > *y {
                Coordinate2d::new(2 * y - point.row(), point.column())
            } else {
                point
            }
            Fold::Left(x) => if point.column() > *x {
                Coordinate2d::new(point.row(), 2 * x - point.column())
            } else {
                point
            }
        }
    }
}

impl ParseYolo for Fold {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        stream.expect("fold along ");
        if stream.try_consume("y=") {
            Self::Up(stream.parse_yolo())
        } else {
            stream.expect("x=");
            Self::Left(stream.parse_yolo())
        }
    }
}


struct Dot {
    x: isize,
    y: isize,
}

impl Dot {
    fn to_coordinate(self) -> Coordinate2d {
        Coordinate2d::new(self.y, self.x)
    }
}

impl ParseYolo for Dot {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        let x = stream.parse_yolo();
        stream.expect(",");
        let y = stream.parse_yolo();
        Self { x, y }
    }
}


#[cfg(test)]
mod tests {
    use unindent::unindent;
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 17);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        let expected = unindent("
            #####
            #...#
            #...#
            #...#
            #####
            .....
            .....
        ");
        assert_eq!(result, expected);
    }

    fn data() -> InputData {
        InputData::from_string("
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
        ")
    }
}