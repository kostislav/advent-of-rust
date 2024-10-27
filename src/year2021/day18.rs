use std::cmp::max;
use std::ops::AddAssign;

use itertools::Itertools;

use parse_yolo_derive::ParseYolo;

use crate::input::{InputData, ParseStream, ParseYolo};

pub fn part_1(input: &InputData) -> u64 {
    let mut number_iterator = input.lines_as::<InputPair>();
    let mut sum = MutableSnailfishNumber::new(&number_iterator.next().unwrap());

    for snailfish_number in number_iterator {
        sum += &snailfish_number;
    }

    sum.magnitude()
}

pub fn part_2(input: &InputData) -> u64 {
    let numbers = input.lines_as::<InputPair>().collect_vec();
    let mut max_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let mut number = MutableSnailfishNumber::new(&numbers[i]);
                number += &numbers[j];
                max_magnitude = max(max_magnitude, number.magnitude());
            }
        }
    }
    max_magnitude
}

#[derive(ParseYolo)]
#[pattern("[{},{}]")]
struct InputPair {
    left: InputPairElement,
    right: InputPairElement,
}

#[derive(ParseYolo)]
enum InputPairElement {
    Number(u32),
    Pair(Box<InputPair>),
}

struct MutableSnailfishNumber {
    tree: [TreeNode; 64],
}

impl MutableSnailfishNumber {
    pub fn new(number: &InputPair) -> Self {
        let mut tree = [TreeNode::Nothing; 64];
        let mut result = Self { tree };
        result.populate_with_pair(number, 16);
        result
    }

    pub fn magnitude(&self) -> u64 {
        self.magnitude_inner(16)
    }

    fn magnitude_inner(&self, index: usize) -> u64 {
        match self.tree[index] {
            TreeNode::Inner => {
                let (left_child, right_child) = Self::child_indexes(index);
                3 * self.magnitude_inner(left_child) + 2 * self.magnitude_inner(right_child)
            }
            TreeNode::Leaf(value) => value as u64,
            TreeNode::Nothing => panic!("Should not happen"),
        }
    }

    fn populate_with_pair(&mut self, input: &InputPair, index: usize) {
        let (left_child_index, right_child_index) = Self::child_indexes(index);
        self.tree[index] = TreeNode::Inner;
        self.populate_with_element(&input.left, left_child_index);
        self.populate_with_element(&input.right, right_child_index);
    }

    fn populate_with_element(&mut self, input: &InputPairElement, index: usize) {
        match input {
            InputPairElement::Number(value) => {
                self.tree[index] = TreeNode::Leaf(*value);
            }
            InputPairElement::Pair(pair) => {
                self.populate_with_pair(pair, index);
            }
        }
    }

    fn parent_index(index: usize) -> usize {
        let low_bits = index ^ (index - 1);
        (index & !low_bits) | (low_bits + 1)
    }

    fn child_indexes(index: usize) -> (usize, usize) {
        let low_bits = index ^ (index - 1);
        let child_offset = (low_bits + 1) >> 2;
        (index - child_offset, index + child_offset)
    }

    fn explode(&mut self, index: usize) -> Option<usize> {
        let (left_child, right_child) = Self::child_indexes(index);
        let left_value = self.tree[left_child].leaf_value();
        let right_value = self.tree[right_child].leaf_value();
        let mut left_split_index = None;
        for i in (0..index - 2).rev() {
            if let TreeNode::Leaf(left_neighbor) = &mut self.tree[i] {
                *left_neighbor += left_value;
                if *left_neighbor >= 10 {
                    left_split_index = Some(i);
                }
                break;
            }
        }
        for i in (index + 2)..64 {
            if let TreeNode::Leaf(right_neighbor) = &mut self.tree[i] {
                *right_neighbor += right_value;
                break;
            }
        }
        self.tree[index] = TreeNode::Leaf(0);
        self.tree[left_child] = TreeNode::Nothing;
        self.tree[right_child] = TreeNode::Nothing;
        left_split_index
    }

    fn split_and_explode(&mut self, index: usize) -> Option<usize> {
        let value = self.tree[index].leaf_value();
        let left_value = value / 2;
        let right_value = value - left_value;
        let (left_child, right_child) = Self::child_indexes(index);
        self.tree[left_child] = TreeNode::Leaf(left_value);
        self.tree[right_child] = TreeNode::Leaf(right_value);
        self.tree[index] = TreeNode::Inner;
        if (left_child & 1) == 1 {
            self.explode(index)
        } else if left_value >= 10 {
            Some(left_child)
        } else {
            None
        }
    }
}

impl AddAssign<&InputPair> for MutableSnailfishNumber {
    fn add_assign(&mut self, rhs: &InputPair) {
        self.populate_with_pair(rhs, 48);
        self.tree[32] = TreeNode::Inner;
        for i in (1..64).step_by(2) {
            if matches!(self.tree[i], TreeNode::Leaf(_)) {
                self.explode(Self::parent_index(i));
            }
        }
        let mut i = 0;
        while i < 64 {
            if let TreeNode::Leaf(value) = self.tree[i] {
                if value >= 10 {
                    if let Some(next_i) = self.split_and_explode(i) {
                        i = next_i;
                        continue;
                    }
                }
            }
            i += 1;
        }
        for i in 1..32 {
            self.tree[i] = self.tree[i << 1];
        }
        for i in 32..64 {
            self.tree[i] = TreeNode::Nothing;
        }
    }
}

#[derive(Copy, Clone)]
enum TreeNode {
    Inner,
    Leaf(u32),
    Nothing,
}

impl TreeNode {
    fn leaf_value(&self) -> u32 {
        match self {
            TreeNode::Leaf(value) => *value,
            _ => panic!("Not a leaf node"),
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

        assert_eq!(result, 4140);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 3993);
    }

    fn data() -> InputData {
        InputData::from_string("
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ")
    }
}