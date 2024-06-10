use crate::array::{Array2d, Coordinate2d};
use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    let mut cavern = OctopusCavern::from_input(input);
    (0..100)
        .map(|_| cavern.run_step())
        .sum()
}

pub fn part_2(input: &InputData) -> i64 {
    let mut cavern = OctopusCavern::from_input(input);
    let mut step = 1;
    loop {
        let num_flashes = cavern.run_step();

        if num_flashes == 100 {
            return step;
        } else {
            step += 1;
        }
    }
}

struct OctopusCavern {
    octopuses: Array2d<u8>,
    flashes: Vec<Coordinate2d>,
}

impl OctopusCavern {
    pub fn from_input(input: &InputData) -> Self {
        Self {
            octopuses: Array2d::from_transformed_input(input, |c| c - b'0'),
            flashes: Vec::with_capacity(1000),
        }
    }

    pub fn run_step(&mut self) -> usize {
        self.octopuses.for_each_mut(|octopus, energy| {
            *energy += 1;
            if *energy == 10 {
                self.flashes.push(octopus);
            }
        });

        let mut i = 0;
        while i < self.flashes.len() {
            let octopus = self.flashes[i];
            for delta_rows in [-1, 0, 1] {
                for delta_columns in [-1, 0, 1] {
                    let neighbor = Coordinate2d::new(octopus.row() + delta_rows, octopus.column() + delta_columns);
                    if neighbor != octopus && self.octopuses.is_inside(&neighbor) {
                        self.octopuses[neighbor] += 1;
                        if self.octopuses[neighbor] == 10 {
                            self.flashes.push(neighbor);
                        }
                    }
                }
            }
            i += 1;
        }

        let num_flashes = self.flashes.len();
        self.flashes.iter().for_each(|&octopus| self.octopuses[octopus] = 0);
        self.flashes.clear();

        num_flashes
    }
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

        assert_eq!(result, 195);
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