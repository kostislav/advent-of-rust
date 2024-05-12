use itertools::Itertools;
use crate::input::{HashableIteratorExtras, InputData, IteratorYoloParsing};

pub fn part_1(input: &InputData) -> usize {
    count_lanternfish(input, 80)
}

pub fn part_2(input: &InputData) -> usize {
    count_lanternfish(input, 256)
}

fn count_lanternfish(input: &InputData, num_days: usize) -> usize {
    let mut counts = [0 as usize; 9];
    for state in input.whole().trim().split(',').parse_yolo::<usize>() {
        counts[state] += 1;
    }

    let mut index = 0;
    for _ in 0..num_days {
        let count = counts[index];
        counts[(index + 9) % 9] = count;
        counts[(index + 7) % 9] += count;
        index = (index + 1) % 9;
    }

    counts.iter().sum()
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn count_lanternfish_works() {
        assert_eq!(count_lanternfish(&data(), 18), 26);
        assert_eq!(count_lanternfish(&data(), 80), 5934);
    }

    fn data() -> InputData {
        InputData::from_string("
            3,4,3,1,2
        ")
    }
}