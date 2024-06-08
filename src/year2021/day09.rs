use std::collections::VecDeque;

use ahash::HashSet;
use derive_new::new;
use itertools::Itertools;

use crate::input::{CopyableIteratorExtras, InputData, OrdIteratorExtras};

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
    let mut basins: Vec<Basin> = Vec::new();
    let mut previous_line: VecDeque<BasinSlice> = VecDeque::new();
    for line in input.lines() {
        let mut current_line: VecDeque<BasinSlice> = VecDeque::new();
        for (slice_start, slice_end_exclusive) in basin_slices(&line) {
            let mut basin_id: Option<usize> = None;
            while let Some(previous_line_slice) = previous_line.front() {
                if previous_line_slice.end_exclusive <= slice_start {
                    previous_line.pop_front();
                } else {
                    break;
                }
            }
            for previous_line_slice in previous_line.iter() {
                if previous_line_slice.start >= slice_end_exclusive {
                    break;
                } else {
                    if let Some(basin_id) = basin_id {
                        if basin_id != previous_line_slice.basin_id {
                            basins[basin_id].merged_with.insert(previous_line_slice.basin_id);
                            basins[previous_line_slice.basin_id].is_child = true;
                        }
                    } else {
                        basin_id = Some(previous_line_slice.basin_id);
                    }
                }
            }
            if basin_id.is_none() {
                basin_id = Some(basins.len());
                basins.push(Basin::default());
            }
            let basin_id = basin_id.unwrap();
            basins[basin_id].size += slice_end_exclusive - slice_start;
            current_line.push_back(BasinSlice::new(slice_start, slice_end_exclusive, basin_id));
        }
        previous_line = current_line;
    }
    basins.iter().enumerate()
        .filter_map(|(basin_id, basin)|
            if basin.is_child {
                None
            } else {
                Some(recursive_basin_size(&basins, basin_id))
            }
        )
        .largest_n(3)
        .product()
}

fn recursive_basin_size(basins: &Vec<Basin>, basin_id: usize) -> usize {
    let basin = &basins[basin_id];
    basin.size + basin.merged_with.iter().map(|&it| recursive_basin_size(basins, it)).sum::<usize>()
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

#[derive(Default)]
struct Basin {
    size: usize,
    merged_with: HashSet<usize>,
    is_child: bool,
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