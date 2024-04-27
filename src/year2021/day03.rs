use itertools::Itertools;
use tailcall::tailcall;
use crate::input::InputData;

pub fn part_1(input: &InputData) -> u64 {
    let mut lines = input.lines().peekable();
    let mut counters: Vec<usize> = vec![0; lines.peek().unwrap().len()];
    let mut num_lines = 0;
    for line in lines {
        line.char_indices()
            .filter(|(_, c)| *c == '1')
            .for_each(|(i, _)| counters[i] += 1);
        num_lines += 1;
    }
    let gamma_rate = counters.iter()
        .fold(0, |acc, count|
            (acc << 1) + (*count > num_lines / 2) as u64
        );
    let epsilon_rate = (1 << counters.len()) - 1 - gamma_rate;

    gamma_rate * epsilon_rate
}

pub fn part_2(input: &InputData) -> u64 {
    let lines = input.lines().sorted().collect_vec();
    let oxygen_generator_rating = find_value(lines.as_slice(), 0, |a, b| a > b);
    let co2_scrubber_rating = find_value(lines.as_slice(), 0, |a, b| a <= b);
    oxygen_generator_rating * co2_scrubber_rating
}


#[tailcall]
fn find_value<F: Fn(usize, usize) -> bool>(values: &[&str], index: usize, comparator: F) -> u64 {
    if let &[value] = values {
        u64::from_str_radix(value, 2).unwrap()
    } else {
        let split_index = values.partition_point(|value| value.as_bytes()[index] == b'0');
        let next = if comparator(split_index, values.len() / 2) {
            0..split_index
        } else {
            split_index..values.len()
        };
        find_value(&values[next], index + 1, comparator)
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