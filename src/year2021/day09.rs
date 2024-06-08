use std::collections::VecDeque;

use ahash::HashSet;
use derive_new::new;
use itertools::Itertools;

use crate::input::{CopyableIteratorExtras, InputData, OrdIteratorExtras, VecDequeExtras};

pub fn part_1(input: &InputData) -> u64 {
    input.lines()
        .peek_around_window()
        .map(|(previous_line, current_line, next_line)|
            current_line.iter().copied()
                .peek_around_window()
                .enumerate()
                .filter_map(|(i, (previous_char, current_char, next_char))| {
                    let lowest_neighbor = [previous_char, next_char, previous_line.map(|it| it[i]), next_line.map(|it| it[i])]
                        .into_iter()
                        .flatten()
                        .min()
                        .unwrap();
                    if current_char < lowest_neighbor {
                        Some(1 + (current_char - b'0') as u64)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        )
        .sum()
}

pub fn part_2(input: &InputData) -> usize {
    let mut basins = BasinForest::new();
    let mut current_line: VecDeque<BasinSlice> = VecDeque::new();
    let mut previous_line: VecDeque<BasinSlice> = VecDeque::new();
    for line in input.lines() {
        for (slice_start, slice_end_exclusive) in basin_slices(&line) {
            let mut basin_id: Option<usize> = None;
            previous_line.pop_front_while(|previous_line_slice| previous_line_slice.end_exclusive <= slice_start);
            previous_line.iter()
                .take_while(|previous_line_slice| previous_line_slice.start < slice_end_exclusive)
                .for_each(|previous_line_slice| {
                    if let Some(basin_id) = basin_id {
                        if basin_id != previous_line_slice.basin_id {
                            basins.merge(basin_id, previous_line_slice.basin_id);
                        }
                    } else {
                        basin_id = Some(previous_line_slice.basin_id);
                    }
                });
            let slice_size = slice_end_exclusive - slice_start;
            let basin_id = match basin_id {
                None => basins.insert_new(slice_size),
                Some(basin_id) => {
                    basins.increase_size(basin_id, slice_size);
                    basin_id
                }
            };
            current_line.push_back(BasinSlice::new(slice_start, slice_end_exclusive, basin_id));
        }
        std::mem::swap(&mut current_line, &mut previous_line);
        current_line.clear();
    }
    basins.root_basin_sizes()
        .largest_n(3)
        .product()
}

fn basin_slices(line: &[u8]) -> impl Iterator<Item=(usize, usize)> + '_ {
    let mut index = 0;
    std::iter::from_fn(move || {
        while index < line.len() && line[index] == b'9' {
            index += 1;
        }
        if index == line.len() {
            None
        } else {
            let start_index = index;
            while index < line.len() && line[index] != b'9' {
                index += 1;
            }
            Some((start_index, index))
        }
    })
}

struct BasinForest {
    basins: Vec<Basin>,
}

impl BasinForest {
    pub fn new() -> Self {
        Self { basins: Vec::new() }
    }

    pub fn merge(&mut self, basin_id_1: usize, basin_id_2: usize) {
        self.basins[basin_id_1].merged_with.insert(basin_id_2);
        self.basins[basin_id_2].is_child = true;
    }

    pub fn insert_new(&mut self, size: usize) -> usize {
        let basin_id = self.basins.len();
        self.basins.push(Basin::new(size));
        basin_id
    }

    pub fn increase_size(&mut self, basin_id: usize, delta: usize) {
        self.basins[basin_id].size += delta;
    }

    pub fn root_basin_sizes(&self) -> impl Iterator<Item=usize> + '_ {
        self.basins.iter().enumerate()
            .filter_map(|(basin_id, basin)|
                if basin.is_child {
                    None
                } else {
                    Some(self.recursive_basin_size(basin_id))
                }
            )
    }

    fn recursive_basin_size(&self, basin_id: usize) -> usize {
        let basin = &self.basins[basin_id];
        basin.size + basin.merged_with.iter().map(|&it| self.recursive_basin_size(it)).sum::<usize>()
    }
}

struct Basin {
    size: usize,
    merged_with: HashSet<usize>,
    is_child: bool,
}

impl Basin {
    fn new(size: usize) -> Self {
        Self {
            size,
            merged_with: HashSet::default(),
            is_child: false,
        }
    }
}

#[derive(new)]
struct BasinSlice {
    start: usize,
    end_exclusive: usize,
    basin_id: usize,
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 15);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 1134);
    }

    fn data() -> InputData {
        InputData::from_string("
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ")
    }
}