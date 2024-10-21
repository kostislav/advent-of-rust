use std::hash::{Hash, Hasher};
use std::ops::Add;
use ahash::{HashMap, HashMapExt};

use crate::graph::shortest_path;
use crate::input::{DefaultIteratorExtras, InputData};

const ENERGY_COSTS: [usize; 4] = [1, 10, 100, 1000];

pub fn part_1(input: &InputData) -> usize {
    solve(input, &[])
}

pub fn part_2(input: &InputData) -> usize {
    solve(
        input,
        &[
            ['D', 'C', 'B', 'A'],
            ['D', 'B', 'A', 'C'],
        ]
    )
}

fn solve(input: &InputData, extra: &[[char; 4]]) -> usize {
    let starting_state = State::from_input(input, extra);

    shortest_path(
        starting_state,
        |state| state.is_done(),
        HashMap::new(),
        |state| state.next_states(2 + extra.len()),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    hallway: Hallway,
    side_rooms: [SideRoom; 4],
}

impl State {
    pub fn from_input(input: &InputData, extra: &[[char; 4]]) -> Self {
        Self {
            hallway: Hallway::default(),
            side_rooms: (0..4)
                .map(|i| SideRoom::from_input(input, i, extra))
                .collect_array(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.hallway.is_empty() && self.side_rooms.iter().all(|room| room.has_no_visitors())
    }

    fn next_states(&self, side_room_depth: usize) -> impl Iterator<Item=(State, usize)> {
        let mut neighbors = heapless::Vec::<_, 56>::new();
        for i in 0..4 {
            let side_room = self.side_rooms[i];
            if let Some(top_visitor) = side_room.top_visitor() {
                let target_room_index = top_visitor.as_int() as usize;
                let starting_room_offset = Hallway::room_offset(i);
                let target_room_offset = Hallway::room_offset(target_room_index);
                let target_room = &self.side_rooms[target_room_index];
                if target_room.has_no_visitors() && self.hallway.is_path_free(starting_room_offset, target_room_offset) {
                    let mut new_side_rooms = self.side_rooms.clone();
                    new_side_rooms[i] = new_side_rooms[i].pop_top_visitor();
                    new_side_rooms[target_room_index] = new_side_rooms[target_room_index].plus_completed();
                    let reduced = Self {
                        hallway: self.hallway,
                        side_rooms: new_side_rooms,
                    };
                    let path_length_to_hallway = 1 + side_room_depth - side_room.num_amphipods();
                    let energy = ENERGY_COSTS[target_room_index] * (path_length_to_hallway + starting_room_offset.abs_diff(target_room_offset) + side_room_depth - target_room.num_amphipods());
                    neighbors.push((reduced, energy)).unwrap();
                    break;
                }
            }
        }
        for i in 0..15 {
            if let Some(amphipod) = self.hallway.get(i) {
                let target_room_index = amphipod.as_int() as usize;
                let target_room_offset = Hallway::room_offset(target_room_index);
                let target_room = &self.side_rooms[target_room_index];
                if target_room.has_no_visitors() && self.hallway.is_path_free(i, target_room_offset) {
                    let mut new_side_rooms = self.side_rooms.clone();
                    new_side_rooms[target_room_index] = new_side_rooms[target_room_index].plus_completed();
                    let reduced = Self {
                        hallway: self.hallway.remove(i),
                        side_rooms: new_side_rooms,
                    };
                    let energy = ENERGY_COSTS[target_room_index] * (i.abs_diff(target_room_offset) + side_room_depth - target_room.num_amphipods());
                    neighbors.push((reduced, energy)).unwrap();
                    break;
                }
            }
        }
        if neighbors.is_empty() {
            for i in 0..4 {
                let side_room = self.side_rooms[i];
                if let Some(top_visitor) = side_room.top_visitor() {
                    let side_room_offset = Hallway::room_offset(i);
                    let path_length_to_hallway = 1 + side_room_depth - side_room.num_amphipods();
                    for j in (0..side_room_offset).rev() {
                        if self.hallway.is_free(j) {
                            if j != 2 && j != 4 && j != 6 && j != 8 {
                                let mut new_side_rooms = self.side_rooms.clone();
                                new_side_rooms[i] = new_side_rooms[i].pop_top_visitor();
                                let reduced = Self {
                                    hallway: self.hallway.plus(top_visitor, j),
                                    side_rooms: new_side_rooms,
                                };
                                let energy = ENERGY_COSTS[top_visitor.as_int() as usize] * (j.abs_diff(side_room_offset) + path_length_to_hallway);
                                neighbors.push((reduced, energy)).unwrap();
                            }
                        } else {
                            break;
                        }
                    }
                    for j in side_room_offset..11 {
                        if self.hallway.is_free(j) {
                            if j != 2 && j != 4 && j != 6 && j != 8 {
                                let mut new_side_rooms = self.side_rooms.clone();
                                new_side_rooms[i] = new_side_rooms[i].pop_top_visitor();
                                let reduced = Self {
                                    hallway: self.hallway.plus(top_visitor, j),
                                    side_rooms: new_side_rooms,
                                };
                                let energy = ENERGY_COSTS[top_visitor.as_int() as usize] * (j.abs_diff(side_room_offset) + path_length_to_hallway);
                                neighbors.push((reduced, energy)).unwrap();
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        neighbors.into_iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct Hallway(u64);

impl Hallway {
    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn room_offset(room_index: usize) -> usize {
        2 + room_index * 2
    }

    fn is_path_free(&self, start: usize, end: usize) -> bool {
        let (lower, upper) = if start < end {
            (start, end)
        } else {
            (end, start)
        };
        let mask = ((1 << ((upper - lower - 1) * 4)) - 1) << ((lower + 1) * 4);
        self.0 & mask == 0
    }

    fn is_free(&self, index: usize) -> bool {
        ((self.0 >> (index * 4)) & 0xF) == 0
    }

    fn get(&self, index: usize) -> Option<Amphipod> {
        let value = (self.0 >> (index * 4)) & 0xF;
        if value == 0 {
            None
        } else {
            Some(Amphipod::from_int(value - 1))
        }
    }

    fn plus(&self, amphipod: Amphipod, index: usize) -> Self {
        Self(self.0 | (amphipod.as_int() + 1) << (index * 4))
    }

    fn remove(&self, index: usize) -> Self {
        Self(self.0 & !(0xF << (index * 4)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct SideRoom {
    visitors: VisitorStack,
    num_completed: u8,
}

impl SideRoom {
    fn from_input(input: &InputData, room_index: usize, extra: &[[char; 4]]) -> Self {
        let chars = input.raw();
        let top_position = 31 + 2 * room_index;
        let bottom_amphipod = Amphipod::from_char(chars[top_position + 14]);
        let top_amphipod = Amphipod::from_char(chars[top_position]);
        let mut num_completed = 0;
        let mut visitors = VisitorStack::default();
        if bottom_amphipod.is_home_in(room_index) {
            num_completed += 1;
        } else {
            visitors = visitors.plus(bottom_amphipod);
        }
        for row in extra.iter().rev() {
            visitors = visitors.plus(Amphipod::from_char(row[room_index] as u8));
        }
        visitors = visitors.plus(top_amphipod);
        Self { visitors, num_completed }
    }

    fn has_no_visitors(&self) -> bool {
        self.visitors.is_empty()
    }

    fn top_visitor(&self) -> Option<Amphipod> {
        self.visitors.peek()
    }

    fn pop_top_visitor(&self) -> Self {
        Self {
            visitors: self.visitors.pop(),
            num_completed: self.num_completed,
        }
    }

    fn num_amphipods(&self) -> usize {
        self.num_completed as usize + self.visitors.len()
    }

    fn plus_completed(&self) -> Self {
        Self {
            visitors: self.visitors.clone(),
            num_completed: self.num_completed + 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct VisitorStack {
    length: u8,
    values: u8,
}

impl VisitorStack {
    fn plus(&self, amphipod: Amphipod) -> VisitorStack {
        Self {
            length: self.length + 1,
            values: self.values | (amphipod.as_int() << (self.length * 2)) as u8,
        }
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn len(&self) -> usize {
        self.length as usize
    }

    fn peek(&self) -> Option<Amphipod> {
        if self.length == 0 {
            None
        } else {
            Some(Amphipod::from_int(((self.values >> (self.length - 1) * 2) & 3) as u64))
        }
    }

    fn pop(&self) -> Self {
        Self{
            length: self.length - 1,
            values: self.values & !(3 << ((self.length - 1) * 2))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Amphipod(u8);

impl Amphipod {
    fn from_char(c: u8) -> Self {
        Self(c - b'A')
    }

    fn from_int(i: u64) -> Self {
        Self(i as u8)
    }

    fn as_int(&self) -> u64 {
        self.0 as u64
    }

    fn is_home_in(&self, side_room_index: usize) -> bool {
        self.0 as usize == side_room_index
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

        assert_eq!(result, 44169);
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