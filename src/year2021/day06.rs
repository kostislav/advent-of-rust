use crate::input::{InputData, WrappingArray};

#[allow(dead_code)]
pub fn part_1(input: &InputData) -> usize {
    count_lanternfish(input, 80)
}

#[allow(dead_code)]
pub fn part_2(input: &InputData) -> usize {
    count_lanternfish(input, 256)
}

fn count_lanternfish(input: &InputData, num_days: usize) -> usize {
    let mut counts = WrappingArray::<usize, 9>::default();
    for state in input.stream().parse_iter::<usize>(",") {
        counts[state] += 1;
    }

    for day in 0..num_days {
        counts[day + 7] += counts[day + 9];
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
        InputData::from_string("3,4,3,1,2")
    }
}