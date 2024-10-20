use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::ops::Add;
use ahash::{HashMap, HashMapExt};

use crate::collections::SmallIntSet;
use crate::graph::shortest_path;
use crate::input::InputData;

const ENERGY_COSTS: [usize; 5] = [0, 1, 10, 100, 1000];

pub fn part_1(input: &InputData) -> usize {
    let starting_state = State::from_input(input);
    let edges = [
        (1, 2),
        (3, 4),
        (5, 6),
        (7, 8),
        (9, 10),
        (14, 15),
        (10, 16),
        (16, 11),
        (16, 1),
        (11, 17),
        (17, 12),
        (3, 17),
        (12, 18),
        (18, 13),
        (5, 18),
        (13, 19),
        (19, 14),
        (7, 19),
    ];
    let mut paths = [[Path::SAME_TILE; 16]; 16];
    for i in 1..16 {
        for j in 1..16 {
            if i != j {
                calculate_path(SmallIntSet::empty(), i, j, &mut paths, &edges);
            }
        }
    }

    shortest_path(
        starting_state,
        State::final_state(),
        HashMap::new(),
        |state| state.next_states(&paths),
    )
}

pub fn part_2(input: &InputData) -> i64 {
    0
}

fn calculate_path(visited: SmallIntSet, start: usize, end: usize, paths: &mut [[Path; 16]; 16], edges: &[(usize, usize)]) -> Path {
    let mut smallest = Path::SAME_TILE;
    for edge in edges {
        let maybe_other = if edge.0 == start {
            Some(edge.1)
        } else if edge.1 == start {
            Some(edge.0)
        } else {
            None
        };
        if let Some(other) = maybe_other {
            if !visited.contains(other) {
                let edge = Path::edge(start, other);
                if other == end {
                    smallest = edge;
                    break;
                } else {
                    let rest = calculate_path(visited + start, other, end, paths, edges);
                    if rest != Path::SAME_TILE {
                        smallest = rest.plus(&edge);
                        break;
                    }
                }
            }
        }
    }
    if smallest != Path::SAME_TILE && start < 16 {
        paths[start][end] = smallest;
    }
    smallest
}

#[derive(Copy, Clone, Debug)]
struct State {
    tiles: u64,
    queue: AmphipodQueue,
}

impl State {
    pub fn from_input(input: &InputData) -> Self {
        let chars = input.raw();
        let amphipod_char_positions = [31, 45, 33, 47, 35, 49, 37, 51];
        let amphipods = amphipod_char_positions.map(|position| Amphipod::from_char(chars[position]).as_int());
        let mut tiles = 0;
        let mut queue = AmphipodQueue::empty();
        for i in 0..8 {
            tiles += amphipods[i] << (4 * (i + 1));
            if !((amphipods[i] << 1) == i as u64 + 1) {
                queue = queue.plus((i + 1) as u64);
            }
        }
        Self { tiles, queue }
    }

    pub fn final_state() -> Self {
        Self {
            tiles: (1 << 4) | (1 << 8) | (2 << 12) | (2 << 16) | (3 << 20) | (3 << 24) | (4 << 28) | (4 << 32),
            queue: AmphipodQueue::empty(),
        }
    }

    fn display_tile(&self, index: usize) -> char {
        let value = self.get(index as u64);
        if value == 0 {
            '.'
        } else {
            Amphipod::from_int(value).as_char()
        }
    }

    fn next_states(&self, paths: &[[Path; 16]; 16]) -> impl Iterator<Item=(State, usize)> {
        let mut neighbors = heapless::Vec::<_, 56>::new();
        for i in 0..self.queue.len() {
            let amphipod_position = self.queue.get(i);
            let amphipod = self.get(amphipod_position);
            let lower_home = amphipod * 2;
            let upper_home = lower_home - 1;
            let paths_from_here = &paths[amphipod_position as usize];
            let energy_per_tile = ENERGY_COSTS[amphipod as usize];
            if let Some(path_length) = self.is_path_clear(paths_from_here[lower_home as usize]) {
                let reduced = Self {
                    tiles: (self.tiles & !(0xF << 4 * amphipod_position)) | amphipod << (4 * lower_home),
                    queue: self.queue.remove(i as u64),
                };
                neighbors.push((reduced, energy_per_tile * path_length)).unwrap();
                break;
            }

            if self.get(lower_home) == amphipod {
                if let Some(path_length) = self.is_path_clear(paths_from_here[upper_home as usize]) {
                    let reduced = Self {
                        tiles: (self.tiles & !(0xF << 4 * amphipod_position)) | amphipod << (4 * upper_home),
                        queue: self.queue.remove(i as u64),
                    };
                    neighbors.push((reduced, energy_per_tile * path_length)).unwrap();
                    break;
                }
            }
        }

        if neighbors.is_empty() {
            for i in 0..self.queue.len() {
                let amphipod_position = self.queue.get(i);
                let amphipod = self.get(amphipod_position);
                let paths_from_here = &paths[amphipod_position as usize];
                let energy_per_tile = ENERGY_COSTS[amphipod as usize];

                if amphipod_position <= 8 && ((amphipod_position & 1) == 1 || (self.get(amphipod_position - 1) == 0)) {
                    for j in 9..=15 {
                        if let Some(path_length) = self.is_path_clear(paths_from_here[j]) {
                            let moved = Self {
                                tiles: (self.tiles & !(0xF << 4 * amphipod_position)) | amphipod << (4 * j),
                                queue: self.queue.replace(i as u64, j as u64),
                            };
                            neighbors.push((moved, energy_per_tile * path_length)).unwrap();
                        }
                    }
                }
            }
        }
        neighbors.into_iter()
    }

    fn get(&self, position: u64) -> u64 {
        self.tiles >> (4 * position) & 0xF
    }

    fn is_path_clear(&self, path: Path) -> Option<usize> {
        if (self.tiles & path.0) == 0 {
            Some(path.length() as usize)
        } else {
            None
        }
    }
}

// TODO remove
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n")?;

        f.write_char('#')?;
        f.write_char(self.display_tile(9))?;
        f.write_char(self.display_tile(10))?;
        f.write_char('.')?;
        f.write_char(self.display_tile(11))?;
        f.write_char('.')?;
        f.write_char(self.display_tile(12))?;
        f.write_char('.')?;
        f.write_char(self.display_tile(13))?;
        f.write_char('.')?;
        f.write_char(self.display_tile(14))?;
        f.write_char(self.display_tile(15))?;
        f.write_str("#\n")?;

        f.write_str("###")?;
        f.write_char(self.display_tile(1))?;
        f.write_char('#')?;
        f.write_char(self.display_tile(3))?;
        f.write_char('#')?;
        f.write_char(self.display_tile(5))?;
        f.write_char('#')?;
        f.write_char(self.display_tile(7))?;
        f.write_str("###\n")?;

        f.write_str("  #")?;
        f.write_char(self.display_tile(2))?;
        f.write_char('#')?;
        f.write_char(self.display_tile(4))?;
        f.write_char('#')?;
        f.write_char(self.display_tile(6))?;
        f.write_char('#')?;
        f.write_char(self.display_tile(8))?;
        f.write_str("#  \n")?;
        f.write_str("  #########  \n")
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

impl Eq for State {
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tiles.hash(state)
    }
}

// TODO remove
struct Amphipod(u64);

impl Amphipod {
    fn from_char(c: u8) -> Self {
        Self((c - b'A' + 1) as u64)
    }

    fn from_int(value: u64) -> Self {
        Self(value)
    }

    fn as_int(&self) -> u64 {
        self.0
    }

    fn as_char(&self) -> char {
        (self.0 as u8 + b'A' - 1) as char
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Path(u64);

impl Path {
    const SAME_TILE: Path = Self(0);

    fn edge(start: usize, end: usize) -> Self {
        if end < 16 {
            Self((0xF << (4 * end)) | 1)
        } else {
            Self(1)
        }
    }

    fn plus(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }

    fn length(&self) -> u64 {
        self.0 & 0xF
    }
}

#[derive(Copy, Clone, Debug)]
struct AmphipodQueue(u64);

impl AmphipodQueue {
    fn empty() -> Self {
        Self(0)
    }

    fn plus(&self, index: u64) -> Self {
        Self((self.0 | (index << ((self.len() + 1) * 4))) + 1)
    }

    fn get(&self, position: usize) -> u64 {
        (self.0 >> ((position + 1) * 4)) & 0xF
    }

    fn len(&self) -> usize {
        (self.0 & 0xF) as usize
    }

    fn remove(&self, index: u64) -> Self {
        let lower_mask = (1 << ((index + 1) * 4)) - 1;
        let upper_mask = u64::MAX ^ lower_mask;
        Self((((self.0 >> 4) & upper_mask) | (self.0 & lower_mask)) - 1)
    }

    fn replace(&self, index: u64, new_position: u64) -> Self {
        let shift = (index + 1) * 4;
        Self((self.0 & !(0xF << shift)) | (new_position << shift))
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 12521);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> InputData {
        InputData::from_string("
            #############
            #...........#
            ###B#C#B#D###
              #A#D#C#A#
              #########
        ")
    }
}