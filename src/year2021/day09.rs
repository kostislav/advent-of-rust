use std::collections::VecDeque;
use crate::array::{Array2d, Coordinate2d};
use crate::input::{CopyableIteratorExtras, InputData};

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

pub fn part_2(input: &InputData) -> i64 {
    let heightmap: Array2d<&u8> = input.lines().collect();
    let mut visited = Array2d::empty(heightmap.num_rows(), heightmap.num_columns(), false);
    let mut queue = VecDeque::<Coordinate2d>::new();
    let mut basin_sizes = Vec::new();

    queue.push_back(Coordinate2d::new(0, 0));

    while let Some(point) = queue.pop_front() {
        if !visited[point] {
            if *heightmap[point] == b'9' {
                visited[point] = true;
                for neighbor in [point.up(), point.down(), point.left(), point.right()] {
                    if heightmap.is_inside(&neighbor) && !visited[neighbor] {
                        queue.push_back(neighbor);
                    }
                }
            } else {
                let mut basin_queue = VecDeque::new();
                basin_queue.push_back(point);
                let mut basin_size = 0;

                while let Some(point) = basin_queue.pop_front() {
                    if !visited[point] {
                        visited[point] = true;
                        basin_size += 1;
                        for neighbor in [point.up(), point.down(), point.left(), point.right()] {
                            if heightmap.is_inside(&neighbor) && !visited[neighbor] {
                                if *heightmap[neighbor] == b'9' {
                                    queue.push_back(neighbor);
                                } else {
                                    basin_queue.push_back(neighbor);
                                }
                            }
                        }
                    }
                }

                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).product()
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