use crate::input::{DefaultIteratorExtras, InputData, IteratorYoloParsing, ParseYolo};

pub fn part_1(input: &InputData) -> usize {
    input.lines()
        .parse_yolo::<PuzzleInput>()
        .flat_map(|part| part.output_values)
        .filter(|digit| digit.len() != 5 && digit.len() != 6)
        .count()
}

pub fn part_2(input: &InputData) -> i64 {
    0
}

struct PuzzleInput<'a> {
    signal_patterns: [&'a str; 10],
    output_values: [&'a str; 4],
}

impl<'a> ParseYolo<'a> for PuzzleInput<'a> {
    fn parse(s: &'a str) -> Self {
        let (signal_patterns, output_values) = s.split_once(" | ").unwrap();
        Self {
            signal_patterns: signal_patterns.split(' ').collect_array(),
            output_values: output_values.split(' ').collect_array(),
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

        assert_eq!(result, 26);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
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