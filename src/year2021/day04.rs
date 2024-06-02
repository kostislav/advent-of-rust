use ahash::AHashMap;
use derive_new::new;
use itertools::Itertools;

use crate::array::Array2d;
use crate::input::{InputData, IteratorExtras, U8IteratorExtras, U8SliceExtras};

pub fn part_1(input: &InputData) -> u64 {
    score_boards(input)
        .min_by_key(|board| board.winning_turn).unwrap()
        .score
}

pub fn part_2(input: &InputData) -> u64 {
    score_boards(input)
        .max_by_key(|board| board.winning_turn).unwrap()
        .score
}

#[derive(new)]
struct ProcessedBoard {
    winning_turn: usize,
    score: u64,
}

fn score_boards(input: &InputData) -> impl Iterator<Item=ProcessedBoard> + '_ {
    let mut lines = input.lines().peekable();
    let turn_per_number: AHashMap<_, _> = lines.next().unwrap().stream().parse_iter::<u64>(",")
        .enumerate_as_second()
        .collect();
    lines.next();
    lines
        .map_chunks(move |chunk| {
            chunk.into_iter().map(|line| line.stream().parse_iter_right_aligned::<u64>()
                .map(|number| (number, turn_per_number[&number]))
                .collect_vec()
            ).collect::<Array2d<(u64, usize)>>()
        })
        .map(|board| {
            let (last_number, winning_turn) = board.columns().chain(board.rows())
                .map(|row_or_column| *row_or_column.iter().max_by_key(|(_, turn)| turn).unwrap())
                .min_by_key(|(_, turn)| *turn).unwrap();
            let remaining_number_sum: u64 = board.iter().filter(|(_, turn)| *turn > winning_turn).map(|(number, _)| number).sum();
            ProcessedBoard::new(
                winning_turn,
                remaining_number_sum * last_number,
            )
        })
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