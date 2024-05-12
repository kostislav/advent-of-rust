use std::cmp::{max, Ordering};
use std::iter::successors;

use parse_display::FromStr;

use crate::input::{HashableIteratorExtras, InputData, IteratorYoloParsing};

pub fn part_1(input: &InputData) -> usize {
    input.lines().parse_yolo::<Line2D>()
        .filter(|line| !line.is_diagonal())
        .flat_map(|line| line.covered_points())
        .histogram()
        .into_values()
        .filter(|it| *it >= 2)
        .count()
}

pub fn part_2(input: &InputData) -> usize {
    input.lines().parse_yolo::<Line2D>()
        .flat_map(|line| line.covered_points())
        .histogram()
        .into_values()
        .filter(|it| *it >= 2)
        .count()
}

#[derive(FromStr, PartialEq, Eq, Hash, Clone, Copy)]
#[display("{x},{y}")]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(FromStr)]
#[display("{start} -> {end}")]
struct Line2D {
    start: Point2D,
    end: Point2D,
}

impl Line2D {
    pub fn covered_points(&self) -> Vec<Point2D> {
        let delta_x = sign(self.end.x - self.start.x);
        let delta_y = sign(self.end.y - self.start.y);
        let length = max(self.end.x.abs_diff(self.start.x), self.end.y.abs_diff(self.start.y)) + 1;
        successors(
            Some(self.start),
            |Point2D { x, y }| Some(Point2D::new(x + delta_x, y + delta_y)),
        )
            .take(length as usize)
            .collect()
    }

    pub fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

fn sign(value: i64) -> i64 {
    match value.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 5);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 12);
    }

    fn data() -> InputData {
        InputData::from_string("
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ")
    }
}