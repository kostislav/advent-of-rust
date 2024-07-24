use bstr::ByteSlice;
use itertools::Itertools;

use parse_yolo_derive::ParseYolo;

use crate::collections::U8Map;
use crate::graph::HashIndexer;
use crate::input::{InputData, OrdIteratorExtras, Word};

pub fn part_1(input: &InputData) -> usize {
    run(input, 10)
}

pub fn part_2(input: &InputData) -> usize {
    run(input, 40)
}

fn run(input: &InputData, steps: usize) -> usize {
    let mut stream = input.stream();
    let template: Word = stream.parse_header();
    let mut indexer: HashIndexer<Pair> = HashIndexer::new();
    let mut rules: U8Map<[u8; 2]> = U8Map::new();
    for rule in stream.parse_iter::<InsertionRule>("\n") {
        let pair = Pair::from_bytes(rule.pair.as_bytes());
        let rule_index = indexer.get_or_insert(pair) as u8;
        let children = pair.insert(rule.new_element as u8);
        rules.insert(rule_index, [indexer.get_or_insert(children[0]) as u8, indexer.get_or_insert(children[1]) as u8]);
    }
    let mut current_state = U8Map::new();
    template.as_bytes().iter()
        .tuple_windows()
        .for_each(|(&char_1, &char_2)| current_state.insert(indexer.get_or_insert(Pair::new(char_1, char_2)) as u8, 1));

    for _ in 0..steps {
        let mut next_state = U8Map::new();
        current_state.entries().for_each(|(i, count)| {
            let rule = rules.get(i);
            *next_state.get_mut(rule[0]) += count;
            *next_state.get_mut(rule[1]) += count;
        });
        current_state = next_state;
    }
    let mut counts = U8Map::<usize>::new();
    *counts.get_mut(template.as_bytes()[0]) += 1;
    *counts.get_mut(template.as_bytes().last_byte().unwrap()) += 1;
    current_state.entries().for_each(|(i, count)| {
        let pair = indexer.get_by_index(i as usize).0;
        *counts.get_mut(pair[0]) += count;
        *counts.get_mut(pair[1]) += count;
    });
    let (min, max) = counts.entries()
        .map(|(_, count)| count)
        .min_max_yolo();
    (max - min) / 2
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pair([u8; 2]);

impl Pair {
    fn new(char_1: u8, char_2: u8) -> Self {
        Self([char_1, char_2])
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Self([bytes[0], bytes[1]])
    }

    fn insert(&self, splitter: u8) -> [Pair; 2] {
        [
            Self([self.0[0], splitter]),
            Self([splitter, self.0[1]]),
        ]
    }
}


#[derive(ParseYolo)]
#[pattern("{} -> {}")]
struct InsertionRule<'a> {
    pair: Word<'a>,
    new_element: char,
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 1588);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 2188189693529);
    }

    fn data() -> InputData {
        InputData::from_string("
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        ")
    }
}