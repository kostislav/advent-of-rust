use itertools::Itertools;

use crate::input::{InputData, IteratorParsingUsingFromStr};

pub fn part_1(input: &InputData) -> usize {
    input.lines()
        .parse_yolo::<u64>()
        .tuple_windows()
        .filter(|(previous, current)| current > previous)
        .count()
}

pub fn part_2(input: &InputData) -> usize {
    input.lines()
        .parse_yolo::<u64>()
        .tuple_windows()
        .filter(|(first, _, _, last)| last > first)
        .count()
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 5);
    }

    fn data() -> InputData {
        InputData::from_string("
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ")
    }
}