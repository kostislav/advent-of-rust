use itertools::Itertools;
use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    input.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .tuple_windows::<(_, _)>()
        .filter(|(previous, current)| current > previous)
        .count()
}

pub fn part_2(input: &InputData) -> usize {
    input.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(first, _, _, last)| last > first)
        .count()
}