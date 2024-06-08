use std::collections::VecDeque;
use itertools::Itertools;
use crate::input::{InputData, OrdIteratorExtras};

pub fn part_1(input: &InputData) -> u64 {
    let closers = closers();
    let mut points = [0u16; 128];
    points[b')' as usize] = 3;
    points[b']' as usize] = 57;
    points[b'}' as usize] = 1197;
    points[b'>' as usize] = 25137;
    input.lines()
        .filter_map(|line| {
            let mut stack = VecDeque::with_capacity(line.len());
            for &c in line {
                let closer = closers[c as usize];
                if closer != 0 {
                    stack.push_back(closer);
                } else {
                    if stack.pop_back().unwrap() != c {
                        return Some(points[c as usize] as u64);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part_2(input: &InputData) -> u64 {
    let closers = closers();
    let mut points = [0u8; 128];
    points[b')' as usize] = 1;
    points[b']' as usize] = 2;
    points[b'}' as usize] = 3;
    points[b'>' as usize] = 4;
    input.lines()
        .filter_map(|line| {
            let mut stack = VecDeque::with_capacity(line.len());
            for &c in line {
                let closer = closers[c as usize];
                if closer != 0 {
                    stack.push_back(closer);
                } else {
                    if stack.pop_back().unwrap() != c {
                        return None;
                    }
                }
            }
            Some(
                stack.iter().rev()
                    .fold(0u64, |acc, &c| acc * 5 + points[c as usize] as u64)
            )
        })
        .median()
}

fn closers() -> [u8; 128] {
    let mut closers = [0u8; 128];
    closers[b'(' as usize] = b')';
    closers[b'[' as usize] = b']';
    closers[b'{' as usize] = b'}';
    closers[b'<' as usize] = b'>';
    closers
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