use crate::input::InputData;

pub fn part_1<I: InputData>(input: &I) -> i64 {
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
            (acc << 1) + if *count > num_lines / 2 { 1 } else { 0 }
        );
    let epsilon_rate = (1 << counters.len()) - 1 - gamma_rate;

    gamma_rate * epsilon_rate
}

pub fn part_2<I: InputData>(input: &I) -> i64 {
    0
}


#[cfg(test)]
mod tests {
    use crate::input::StringInputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 198);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> StringInputData {
        StringInputData::new("
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