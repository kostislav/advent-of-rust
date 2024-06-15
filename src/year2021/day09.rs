use std::collections::VecDeque;

use derive_new::new;

use crate::input::{CopyableIteratorExtras, InputData, OrdIteratorExtras, VecDequeExtras};

pub fn part_1(input: &InputData) -> u64 {
    input.lines()
        .peek_around_window()
        .map(|(previous_line, current_line, next_line)| {
            (0..current_line.len())
                .map(|i| {
                    let neighbors = [
                        current_line.get(i - 1).copied(),
                        current_line.get(i + 1).copied(),
                        previous_line.map(|it| it[i]),
                        next_line.map(|it| it[i]),
                    ];
                    let lowest_neighbor = neighbors.into_iter().map(|it| it.unwrap_or(b'9')).min().unwrap();
                    let current_char = current_line[i];
                    if current_char < lowest_neighbor {
                        1 + (current_char - b'0') as u64
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
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
        let (root_id_1, size_1) = self.find_root(basin_id_1);
        let (root_id_2, size_2) = self.find_root(basin_id_2);
        self.basins[root_id_1] = Basin::Root(size_1 + size_2);
        self.basins[root_id_2] = Basin::Child(root_id_1);
    }

    pub fn insert_new(&mut self, size: usize) -> usize {
        let basin_id = self.basins.len();
        self.basins.push(Basin::Root(size));
        basin_id
    }

    pub fn increase_size(&mut self, basin_id: usize, delta: usize) {
        let (root_id, current_size) = self.find_root(basin_id);
        self.basins[root_id] = Basin::Root(current_size + delta);
    }

    pub fn root_basin_sizes(&self) -> impl Iterator<Item=usize> + '_ {
        self.basins.iter()
            .filter_map(|basin|
                match basin {
                    Basin::Root(size) => Some(*size),
                    Basin::Child(_) => None
                }
            )
    }

    fn find_root(&self, basin_id: usize) -> (usize, usize) {
        let mut current_basin_id = basin_id;
        loop {
            match self.basins[current_basin_id] {
                Basin::Root(size) => {
                    return (current_basin_id, size);
                }
                Basin::Child(parent_id) => {
                    current_basin_id = parent_id;
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Basin {
    Root(usize),
    Child(usize),
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