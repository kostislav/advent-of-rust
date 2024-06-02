use itertools::Itertools;
use partition::partition;
use tailcall::tailcall;
use crate::input::InputData;

pub fn part_1(input: &InputData) -> u64 {
    let mut lines = input.lines().peekable();
    let mut counters: Vec<usize> = vec![0; lines.peek().unwrap().len()];
    let mut num_lines = 0;
    for line in lines {
        line.iter().enumerate()
            .filter(|(_, &c)| c == b'1')
            .for_each(|(i, _)| counters[i] += 1);
        num_lines += 1;
    }

    let gamma_rate = counters.iter()
        .map(|count| (*count > num_lines / 2) as u64)
        .fold(0, |acc, bit| (acc << 1) + bit);
    let epsilon_rate = (1 << counters.len()) - 1 - gamma_rate;

    gamma_rate * epsilon_rate
}

pub fn part_2(input: &InputData) -> u64 {
    let mut lines = input.lines().collect_vec();
    let lines_slice = lines.as_mut_slice();
    let oxygen_generator_rating = find_value(lines_slice, 0, |num_zeroes, num_ones| num_zeroes > num_ones);
    let co2_scrubber_rating = find_value(lines_slice, 0, |num_zeroes, num_ones| num_zeroes <= num_ones);
    oxygen_generator_rating * co2_scrubber_rating
}

#[tailcall]
fn find_value<F: Fn(usize, usize) -> bool>(values: &mut [&[u8]], index: usize, comparator: F) -> u64 {
    if let &mut [value] = values {
        value.iter().fold(0, |acc, &digit| (acc << 1) + (digit == b'1') as u64)
    } else {
        let (zeroes, ones) = partition(values, |line| line[index] == b'0');
        let next_value = if comparator(zeroes.len(), ones.len()) {
            ones
        } else {
            zeroes
        };
        find_value(next_value, index + 1, comparator)
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 198);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 230);
    }

    fn data() -> InputData {
        InputData::from_string("
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        ")
    }
}