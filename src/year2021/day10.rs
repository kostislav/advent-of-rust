use std::collections::VecDeque;
use crate::collections::U8Map;

use crate::input::{InputData, OrdIteratorExtras};
use crate::u8_map;

pub fn part_1(input: &InputData) -> u64 {
    let closing_brackets = closing_brackets();
    let mut points = u8_map!(
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
    );
    input.lines()
        .filter_map(|line| {
            let mut stack = VecDeque::with_capacity(line.len());
            for &c in line {
                if let Some(closing_bracket) = closing_brackets.get(c) {
                    stack.push_back(closing_bracket);
                } else if stack.pop_back().unwrap() != c {
                    return Some(points.get(c) as u64);
                }
            }
            None
        })
        .sum()
}

pub fn part_2(input: &InputData) -> u64 {
    let closing_brackets = closing_brackets();
    let mut points = u8_map!(
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
    );
    input.lines()
        .filter_map(|line| {
            let mut stack = VecDeque::with_capacity(line.len());
            for &c in line {
                if let Some(closing_bracket) = closing_brackets.get(c) {
                    stack.push_back(closing_bracket);
                } else if stack.pop_back().unwrap() != c {
                    return None;
                }
            }
            Some(
                stack.iter().rev()
                    .fold(0u64, |acc, &c| acc * 5 + points.get(c) as u64)
            )
        })
        .median()
}

fn closing_brackets() -> U8Map<Option<u8>> {
    u8_map!(
        b'(' => Some(b')'),
        b'[' => Some(b']'),
        b'{' => Some(b'}'),
        b'<' => Some(b'>'),
    )
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 26397);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 288957);
    }

    fn data() -> InputData {
        InputData::from_string("
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        ")
    }
}