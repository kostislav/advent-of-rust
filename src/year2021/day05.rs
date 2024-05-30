use std::cmp::max;
use std::iter::successors;
use derive_new::new;

use parse_display::FromStr;

use crate::input::{HashableIteratorExtras, InputData, IteratorYoloParsing};

pub fn part_1(input: &InputData) -> usize {
    num_intersections(
        input.lines().parse_yolo::<Line2D>()
            .filter(|line| !line.is_diagonal())
    )
}

pub fn part_2(input: &InputData) -> usize {
    num_intersections(
        input.lines().parse_yolo::<Line2D>()
    )
}

fn num_intersections<I: Iterator<Item=Line2D>>(lines: I) -> usize {
    lines
        .flat_map(|line| line.covered_points())
        .histogram()
        .into_values()
        .filter(|it| *it >= 2)
        .count()
}

#[derive(FromStr, PartialEq, Eq, Hash, Clone, Copy, new)]
#[display("{x},{y}")]
struct Point2D {
    x: i64,
    y: i64,
}

#[derive(FromStr)]
#[display("{start} -> {end}")]
struct Line2D {
    start: Point2D,
    end: Point2D,
}

impl Line2D {
    pub fn covered_points(&self) -> impl Iterator<Item=Point2D> {
        let delta_x = (self.end.x - self.start.x).signum();
        let delta_y = (self.end.y - self.start.y).signum();
        let length = max(self.end.x.abs_diff(self.start.x), self.end.y.abs_diff(self.start.y)) + 1;
        successors(
            Some(self.start),
            move |Point2D { x, y }| Some(Point2D::new(x + delta_x, y + delta_y)),
        )
            .take(length as usize)
    }

    pub fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
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