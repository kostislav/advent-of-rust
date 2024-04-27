use itertools::Itertools;
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

pub fn part_2(input: &InputData) -> i64 {
    let lines = input.lines().sorted().collect_vec();
    let oxygen_generator_rating = i64::from_str_radix(most_common(lines.as_slice(), 0), 2).unwrap();
    let co2_scrubber_rating = i64::from_str_radix(least_common(lines.as_slice(), 0), 2).unwrap();
    oxygen_generator_rating * co2_scrubber_rating
}

fn most_common<'a>(values: &[&'a str], index: usize) -> &'a str {
    if values.len() == 1 {
        values[0]
    } else {
        let split_index = values.partition_point(|value| value.as_bytes()[index] == b'0');
        if split_index > values.len() / 2 {
            most_common(&values[..split_index], index + 1)
        } else {
            most_common(&values[split_index..], index + 1)
        }
    }
}

// TODO dedup
fn least_common<'a>(values: &[&'a str], index: usize) -> &'a str {
    if values.len() == 1 {
        values[0]
    } else {
        let split_index = values.partition_point(|value| value.as_bytes()[index] == b'0');
        if split_index > values.len() / 2 {
            least_common(&values[split_index..], index + 1)
        } else {
            least_common(&values[..split_index], index + 1)
        }
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