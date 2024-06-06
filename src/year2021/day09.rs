use crate::input::{CopyableIteratorExtras, InputData};

pub fn part_1(input: &InputData) -> u64 {
    input.lines()
        .peek_around_window()
        .map(|(previous_line, current_line, next_line)|
            current_line.iter().copied()
                .peek_around_window()
                .enumerate()
                .filter_map(|(i, (previous_char, current_char, next_char))| {
                    let lowest_neighbor = [previous_char, next_char, previous_line.map(|it| it[i]), next_line.map(|it| it[i])]
                        .into_iter()
                        .flatten()
                        .min()
                        .unwrap();
                    if current_char < lowest_neighbor {
                        Some(1 + (current_char - b'0') as u64)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        )
        .sum()
}

pub fn part_2(input: &InputData) -> i64 {
    0
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 15);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> InputData {
        InputData::from_string("
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ")
    }
}