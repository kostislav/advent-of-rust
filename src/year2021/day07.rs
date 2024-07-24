use std::cmp::min;

use itertools::Itertools;
use tailcall::tailcall;

use crate::input::{InputData, OrdIteratorExtras};

pub fn part_1(input: &InputData) -> i64 {
    let positions = input.stream().parse_iter::<i64>(",").collect_vec();

    let (&min, &max) = positions.iter().min_max_yolo();

    find_min(
        min,
        max,
        |i| positions.iter().map(|position| (position - i).abs()).sum()
    )
}

pub fn part_2(input: &InputData) -> i64 {
    let positions = input.stream().parse_iter::<i64>(",").collect_vec();

    let (&min, &max) = positions.iter().min_max_yolo();

    find_min(
        min,
        max,
        |i| positions.iter().map(|position| triangular_number((position - i).abs())).sum()
    )
}

fn find_min<F: Fn(i64) -> i64>(lower: i64, upper: i64, function: F) -> i64 {
    bisect((lower, function(lower)), (upper, function(upper)), function)
}

#[tailcall]
fn bisect<F: Fn(i64) -> i64>(lower: (i64, i64), upper: (i64, i64), function: F) -> i64 {
    if upper.0 - lower.0 <= 1 {
        min(lower.1, upper.1)
    } else {
        let middle = lower.0 + (upper.0 - lower.0) / 2;
        let middle_value = function(middle);

        if lower.1 < upper.1 {
            bisect(lower, (middle, middle_value), function)
        } else {
            bisect((middle, middle_value), upper, function)
        }
    }
}

fn triangular_number(n: i64) -> i64 {
    n * (n + 1) / 2
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 37);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 168);
    }

    fn data() -> InputData {
        InputData::from_string("16,1,2,0,4,2,7,1,2,14")
    }
}