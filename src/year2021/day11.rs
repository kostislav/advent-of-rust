use std::collections::VecDeque;

use crate::array::{Array2d, Coordinate2d};
use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    let mut cavern = Array2d::from_transformed_input(input, |c| c - b'0');
    let mut queue = VecDeque::with_capacity(100);
    let mut num_flashes = 0;
    for _ in 0..100 {
        cavern.for_each(|octopus, _| queue.push_back(octopus));

        while let Some(octopus) = queue.pop_front() {
            cavern[octopus] += 1;
            if cavern[octopus] == 10 {
                num_flashes += 1;
                for delta_rows in [-1, 0, 1] {
                    for delta_columns in [-1, 0, 1] {
                        let neighbor = Coordinate2d::new(octopus.row() + delta_rows, octopus.column() + delta_columns);
                        if neighbor != octopus && cavern.is_inside(&neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        cavern.map_in_place(|_, &energy_level|
            if energy_level > 9 {
                0
            } else {
                energy_level
            }
        )
    }

    num_flashes
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

        assert_eq!(result, 1656);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> InputData {
        InputData::from_string("
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        ")
    }
}