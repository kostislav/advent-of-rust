use std::iter::Peekable;
use itertools::Itertools;
use crate::input::{InputData, IteratorYoloParsing};


#[derive(Clone, Copy)]
struct BoardPosition {
    row: u8,
    column: u8,
}

pub fn part_1(input: &InputData) -> u32 {
    score_boards(input)
        .min_by_key(|board| board.draw).unwrap()
        .score
}

pub fn part_2(input: &InputData) -> u32 {
    score_boards(input)
        .max_by_key(|board| board.draw).unwrap()
        .score
}

struct ProcessedBoard {
    draw: u8,
    score: u32,
}

fn score_boards(input: &InputData) -> impl Iterator<Item=ProcessedBoard> + '_ {
    let mut lines = input.lines().peekable();
    let random_order = lines.next().unwrap().split(',').parse_yolo::<u8>().collect_vec();
    lines.next();
    BoardIterator { random_order, lines }
}

struct BoardIterator<I: Iterator> {
    random_order: Vec<u8>,
    lines: Peekable<I>,
}

impl<'a, I: Iterator<Item=&'a str>> Iterator for BoardIterator<I> {
    type Item = ProcessedBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.peek().is_some() {
            Some(process_board(&self.random_order, ChunkLinesIterator { big_iterator: &mut self.lines }))
        } else {
            None
        }
    }
}

struct ChunkLinesIterator<'a, I> {
    big_iterator: &'a mut I,
}

impl<'a, 'b, I: Iterator<Item=&'b str>> Iterator for ChunkLinesIterator<'a, I> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.big_iterator.next() {
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        } else {
            None
        }
    }
}

fn process_board<'a, I: Iterator<Item=&'a str>>(random_order: &[u8], lines: I) -> ProcessedBoard {
    let mut number_positions: [Option<BoardPosition>; 100] = [None; 100];
    let mut remaining_board_total: u32 = 0;
    for (row, line) in lines.enumerate() {
        for (column, number) in line.split_ascii_whitespace().parse_yolo::<u8>().enumerate() {
            number_positions[number as usize] = Some(BoardPosition { row: row as u8, column: column as u8 });
            remaining_board_total += number as u32;
        }
    }
    let mut hits: [u8; 10] = [0; 10];

    for (draw, number) in random_order.iter().copied().enumerate() {
        if let Some(position) = number_positions[number as usize] {
            remaining_board_total -= number as u32;
            for index in [position.row, position.column + 5] {
                let index = index as usize;
                hits[index] += 1;
                if hits[index] == 5 {
                    return ProcessedBoard {
                        draw: draw as u8,
                        score: remaining_board_total * (number as u32)
                    };
                }
            }
        }
    }
    panic!("Board did not win")
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 4512);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 1924);
    }

    fn data() -> InputData {
        InputData::from_string("
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        ")
    }
}