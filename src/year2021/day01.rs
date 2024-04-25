use itertools::Itertools;
use crate::input::{InputData, IteratorParsing};

pub fn part_1(input: &InputData) -> usize {
    input.lines()
        .parse_yolo::<u64>()
        .tuple_windows::<(_, _)>()
        .filter(|(previous, current)| current > previous)
        .count()
}

pub fn part_2(input: &InputData) -> usize {
    input.lines()
        .parse_yolo::<u64>()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(first, _, _, last)| last > first)
        .count()
}