use std::ops::RangeInclusive;

use parse_display::FromStr;

use crate::input::{HashableIteratorExtras, InputData, IteratorYoloParsing};

pub fn part_1(input: &InputData) -> usize {
    let covered_point_histogram = input.lines().parse_yolo::<Line2D>()
        .flat_map(|line| line.covered_points_if_simple())
        .histogram();

    covered_point_histogram.into_values()
        .filter(|it| *it >= 2)
        .count()
}

pub fn part_2(input: &InputData) -> i64 {
    0
}

#[derive(FromStr, PartialEq, Eq, Hash)]
#[display("{x},{y}")]
struct Point2D {
    x: u64,
    y: u64,
}

impl Point2D {
    fn new(x: u64, y: u64) -> Self {
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
    pub fn covered_points_if_simple(&self) -> Vec<Point2D> {
        if self.start.x == self.end.x {
            range_incl_from_unsorted(self.start.y, self.end.y).map(|y| Point2D::new(self.start.x, y)).collect()
        } else if self.start.y == self.end.y {
            range_incl_from_unsorted(self.start.x, self.end.x).map(|x| Point2D::new(x, self.start.y)).collect()
        } else {
            Vec::new()
        }
    }
}

fn range_incl_from_unsorted(value_1: u64, value_2: u64) -> RangeInclusive<u64> {
    if value_1 < value_2 {
        value_1..=value_2
    } else {
        value_2..=value_1
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

        assert_eq!(result, 0);
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