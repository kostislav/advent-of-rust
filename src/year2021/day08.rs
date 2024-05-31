use std::ops::{BitAnd, BitOr, BitXor, Sub};

use derive_more::{BitAnd, BitOr};

use crate::input::{DefaultIteratorExtras, InputData, IteratorExtras, IteratorYoloParsing, ParseYolo};

pub fn part_1(input: &InputData) -> usize {
    input.lines()
        .parse_yolo::<PuzzleInput>()
        .flat_map(|part| part.output_values)
        .filter(|digit| digit.len() != 5 && digit.len() != 6)
        .count()
}

pub fn part_2(input: &InputData) -> u64 {
    input.lines()
        .parse_yolo::<PuzzleInput>()
        .map(|line| {
            let one = unique_digit(&line.signal_patterns, 2);
            let seven = unique_digit(&line.signal_patterns, 3);
            let four = unique_digit(&line.signal_patterns, 4);
            let five_segment_digits: HardDigitSet<3> = filter_by_segment_count(&line.signal_patterns, 5).collect();
            let six_segment_digits: HardDigitSet<3> = filter_by_segment_count(&line.signal_patterns, 6).collect();
            let eight = unique_digit(&line.signal_patterns, 7);

            let a = seven - one;
            let f = six_segment_digits.common_segments() & one;
            let c = one - f;

            let three = five_segment_digits.iter().copied().filter(|digit| *digit & seven == seven).only_element();
            let b = four - three;
            let d = four - b - c - f;
            let g = five_segment_digits.common_segments() - a - d;
            let e = eight - three - b;

            let two = a | c | d | e | g;
            let five = a | b | d | f | g;

            let six = a | b | d | e | f | g;
            let nine = a | b | c | d | f | g;

            let mut unscrambler = [0u8; 256];
            for (i, digit) in [one, two, three, four, five, six, seven, eight, nine].iter().enumerate() {
                unscrambler[digit.0 as usize] = (i + 1) as u8;
            }

            line.output_values.iter()
                .map(|value| unscrambler[value.0 as usize])
                .fold(0u64, |acc, next| acc * 10 + (next as u64))
        })
        .sum()
}

fn filter_by_segment_count(digits: &[SegmentSet], num_segments: usize) -> impl Iterator<Item=SegmentSet> + '_ {
    digits.iter().filter(move |digit| digit.len() == num_segments).copied()
}

fn unique_digit(digits: &[SegmentSet], num_segments: usize) -> SegmentSet {
    filter_by_segment_count(digits, num_segments).only_element()
}

struct HardDigitSet<const N: usize> {
    digits: [SegmentSet; N],
}

impl<const N: usize> HardDigitSet<N> {
    fn iter(&self) -> impl Iterator<Item=&SegmentSet> {
        self.digits.iter()
    }
}

impl<const N: usize> HardDigitSet<N> {
    fn common_segments(&self) -> SegmentSet {
        self.digits.iter().copied().reduce(|acc, segments| acc & segments).unwrap()
    }
}

impl<const N: usize> FromIterator<SegmentSet> for HardDigitSet<N> {
    fn from_iter<T: IntoIterator<Item=SegmentSet>>(iter: T) -> Self {
        Self { digits: iter.into_iter().collect_array() }
    }
}

struct PuzzleInput {
    signal_patterns: [SegmentSet; 10],
    output_values: [SegmentSet; 4],
}

impl<'a> ParseYolo<'a> for PuzzleInput {
    fn parse(s: &'a str) -> Self {
        let (signal_patterns, output_values) = s.split_once(" | ").unwrap();
        Self {
            signal_patterns: signal_patterns.split(' ').parse_yolo().collect_array(),
            output_values: output_values.split(' ').parse_yolo().collect_array(),
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, BitAnd, BitOr)]
struct SegmentSet(u8);

impl SegmentSet {
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl<'a> ParseYolo<'a> for SegmentSet {
    fn parse(s: &'a str) -> Self {
        Self(s.bytes().fold(0, |acc, c| acc | 1 << (c - b'a')))
    }
}

impl Sub for SegmentSet {
    type Output = SegmentSet;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 26);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 61229);
    }

    fn data() -> InputData {
        InputData::from_string("
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ")
    }
}