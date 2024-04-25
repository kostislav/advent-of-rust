use itertools::Itertools;
use crate::input::{InputData, IteratorParsing};

pub fn part_1<I: InputData>(input: &I) -> usize {
    input.lines()
        .parse_yolo::<u64>()
        .tuple_windows::<(_, _)>()
        .filter(|(previous, current)| current > previous)
        .count()
}

pub fn part_2<I: InputData>(input: &I) -> usize {
    input.lines()
        .parse_yolo::<u64>()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(first, _, _, last)| last > first)
        .count()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::input::StringInputData;
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

    fn data() -> StringInputData {
        StringInputData::new(indoc! {"
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
        "}
        )
    }
}