use std::collections::VecDeque;

use crate::input::{InputData, OrdIteratorExtras};

pub fn part_1(input: &InputData) -> u64 {
    let closers = closers();
    let mut points = U8Map::new();
    points.put(b')', 3);
    points.put(b']', 57);
    points.put(b'}', 1197);
    points.put(b'>', 25137);
    input.lines()
        .filter_map(|line| {
            let mut stack = VecDeque::with_capacity(line.len());
            for &c in line {
                let closer = closers.get(c);
                if closer != 0 {
                    stack.push_back(closer);
                } else if stack.pop_back().unwrap() != c {
                    return Some(points.get(c) as u64);
                }
            }
            None
        })
        .sum()
}

pub fn part_2(input: &InputData) -> u64 {
    let closers = closers();
    let mut points = U8Map::new();
    points.put(b')', 1);
    points.put(b']', 2);
    points.put(b'}', 3);
    points.put(b'>', 4);
    input.lines()
        .filter_map(|line| {
            let mut stack = VecDeque::with_capacity(line.len());
            for &c in line {
                let closer = closers.get(c);
                if closer != 0 {
                    stack.push_back(closer);
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

fn closers() -> U8Map<u8> {
    let mut closers = U8Map::new();
    closers.put(b'(', b')');
    closers.put(b'[', b']');
    closers.put(b'{', b'}');
    closers.put(b'<', b'>');
    closers
}

struct U8Map<V> {
    values: [V; 256],
}

impl<V: Default + Copy> U8Map<V> {
    fn new() -> Self {
        Self { values: [V::default(); 256] }
    }

    fn put(&mut self, key: u8, value: V) {
        self.values[key as usize] = value;
    }

    fn get(&self, key: u8) -> V {
        self.values[key as usize]
    }
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