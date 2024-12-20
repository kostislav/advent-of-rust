use std::ops::Sub;

use parse_yolo_derive::ParseYolo;
use crate::collections::SmallIntSet;

use crate::input::{InputData, Word};

pub fn part_1(input: &InputData) -> usize {
    let graph = construct_graph(input);
    find_all_paths(&graph, 0, SmallIntSet::all(), true)
}

pub fn part_2(input: &InputData) -> usize {
    let graph = construct_graph(input);
    find_all_paths(&graph, 0, SmallIntSet::all(), false)
}

fn find_all_paths(graph: &CompactedGraph, index: usize, remaining: SmallIntSet, extra_visited: bool) -> usize {
    if remaining.contains(index) || !extra_visited {
        let (updated_remaining, updated_extra_visited) = if remaining.contains(index) {
            (remaining - index, extra_visited)
        } else {
            (remaining, true)
        };

        let mut num_paths = graph.nodes[index].neighbors[1] as usize;
        for neighbor in 2..graph.num_nodes {
            let neighbor_edge_weight = graph.nodes[index].neighbors[neighbor];
            if neighbor_edge_weight > 0 {
                num_paths += neighbor_edge_weight as usize * find_all_paths(graph, neighbor, updated_remaining, updated_extra_visited);
            }
        }
        num_paths
    } else {
        0
    }
}

fn construct_graph<'a>(input: &'a InputData) -> CompactedGraph {
    let mut small_cave_indexes: Indexer<Word<'a>, 16> = Indexer::new();
    small_cave_indexes.get_or_insert_index(Word::from_str("start"));
    small_cave_indexes.get_or_insert_index(Word::from_str("end"));
    let mut big_cave_indexes: Indexer<Word<'a>, 16> = Indexer::new();

    let mut small_cave_edges = [OriginalGraphNode::default(); 16];
    let mut big_cave_edges = [OriginalGraphNode::default(); 16];

    input.lines_as::<Entry>().for_each(|mut entry| {
        if is_big_cave(&entry.point_2) {
            std::mem::swap(&mut entry.point_1, &mut entry.point_2);
        }

        if is_big_cave(&entry.point_1) {
            let cave_1_index = big_cave_indexes.get_or_insert_index(entry.point_1);
            let cave_2_index = small_cave_indexes.get_or_insert_index(entry.point_2);

            big_cave_edges[cave_1_index].small_neighbors.add(cave_2_index);
            small_cave_edges[cave_2_index].big_neighbors.add(cave_1_index);
        } else {
            let cave_1_index = small_cave_indexes.get_or_insert_index(entry.point_1);
            let cave_2_index = small_cave_indexes.get_or_insert_index(entry.point_2);

            small_cave_edges[cave_1_index].small_neighbors.add(cave_2_index);
            small_cave_edges[cave_2_index].small_neighbors.add(cave_1_index);
        }
    });

    let mut compacted_graph = [CompactedGraphNode::default(); 16];

    for i in 0..small_cave_indexes.len() {
        let neighbors = small_cave_edges[i];
        for j in 0..i {
            let mut num_compacted_neighbors = 0;
            if neighbors.small_neighbors.contains(j) {
                num_compacted_neighbors += 1;
            }
            #[allow(clippy::needless_range_loop)]
            for k in 0..big_cave_indexes.len() {
                if neighbors.big_neighbors.contains(k) && big_cave_edges[k].small_neighbors.contains(j) {
                    num_compacted_neighbors += 1;
                }
            }
            compacted_graph[i].neighbors[j] = num_compacted_neighbors as u8;
            compacted_graph[j].neighbors[i] = num_compacted_neighbors as u8;
        }
        if i >= 2 {
            let mut num_self_edges = 0;
            #[allow(clippy::needless_range_loop)]
            for k in 0..big_cave_indexes.len() {
                if neighbors.big_neighbors.contains(k) && big_cave_edges[k].small_neighbors.contains(i) {
                    num_self_edges += 1;
                }
            }
            compacted_graph[i].neighbors[i] = num_self_edges as u8;
        }
    }

    CompactedGraph { num_nodes: small_cave_indexes.len(), nodes: compacted_graph }
}

fn is_big_cave(cave_name: &Word) -> bool {
    cave_name.as_bytes()[0] <= b'Z'
}


struct Indexer<T, const N: usize> {
    values: heapless::Vec<T, N>,
}

impl<T: Eq, const N: usize> Indexer<T, N> {
    fn new() -> Self {
        Self { values: heapless::Vec::new() }
    }

    fn get_or_insert_index(&mut self, value: T) -> usize {
        for (index, stored) in self.values.iter().enumerate() {
            if *stored == value {
                return index;
            }
        }
        let index = self.values.len();
        self.values.push(value).unwrap_or_else(|_| panic!("Vector is full"));
        index
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}


#[derive(Default, Clone, Copy)]
struct OriginalGraphNode {
    small_neighbors: SmallIntSet,
    big_neighbors: SmallIntSet,
}


#[derive(Default, Clone, Copy)]
struct CompactedGraphNode {
    neighbors: [u8; 16],
}

struct CompactedGraph {
    num_nodes: usize,
    nodes: [CompactedGraphNode; 16],
}


#[derive(ParseYolo)]
#[pattern("{}-{}")]
struct Entry<'a> {
    point_1: Word<'a>,
    point_2: Word<'a>,
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(&small_example()), 10);
        assert_eq!(part_1(&slightly_larger_example()), 19);
        assert_eq!(part_1(&even_larger_example()), 226);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(&small_example()), 36);
        assert_eq!(part_2(&slightly_larger_example()), 103);
        assert_eq!(part_2(&even_larger_example()), 3509);
    }

    fn small_example() -> InputData {
        InputData::from_string("
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        ")
    }

    fn slightly_larger_example() -> InputData {
        InputData::from_string("
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        ")
    }

    fn even_larger_example() -> InputData {
        InputData::from_string("
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        ")
    }
}