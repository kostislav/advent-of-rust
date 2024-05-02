use std::io::BufRead;
use itertools::Itertools;
use crate::input::{InputData, IteratorExtras, IteratorYoloParsing};


#[derive(Clone, Copy)]
struct BoardPosition {
    row: u8,
    column: u8,
}

pub fn part_1(input: &InputData) -> u32 {
    let mut lines = input.lines().peekable();
    let random_order = lines.next().unwrap().split(',').parse_yolo::<u8>().collect_vec();
    lines.next();
    lines.split_on(|line| line.is_empty())
        .map(|board_lines| process_board(&random_order, board_lines))
        .min_by_key(|(draw, _)| *draw).unwrap()
        .1
}

pub fn part_2(input: &InputData) -> u32 {
    let mut lines = input.lines().peekable();
    let random_order = lines.next().unwrap().split(',').parse_yolo::<u8>().collect_vec();
    lines.next();
    lines.split_on(|line| line.is_empty())
        .map(|board_lines| process_board(&random_order, board_lines))
        .max_by_key(|(draw, _)| *draw).unwrap()
        .1
}

fn process_board<'a, I: Iterator<Item=&'a str>>(random_order: &[u8], mut lines: I) -> (u8, u32) {
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
                    return (draw as u8, remaining_board_total * (number as u32));
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