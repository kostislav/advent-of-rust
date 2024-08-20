use std::cmp::max;
use itertools::Itertools;
use parse_yolo_derive::ParseYolo;
use crate::input::{InputData, ParseStream, ParseYolo};

pub fn part_1(input: &InputData) -> u64 {
    let mut number_iterator = input.lines_as::<InputPair>();
    let mut tree = [TreeNode::Nothing; 64];

    number_iterator.next().unwrap().parse_into(&mut tree, 16);

    for snailfish_number in number_iterator {
        add(&mut tree, &snailfish_number);
    }
    magnitude(&tree, 16)
}

pub fn part_2(input: &InputData) -> u64 {
    let mut tree = [TreeNode::Nothing; 64];
    let numbers = input.lines_as::<InputPair>().collect_vec();
    let mut max_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                tree.fill(TreeNode::Nothing);
                numbers[i].parse_into(&mut tree, 16);
                add(&mut tree, &numbers[j]);
                max_magnitude = max(max_magnitude, magnitude(&tree, 16));
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

impl InputPair {
    fn parse_into(&self, tree: &mut [TreeNode], index: usize) {
        let (left_child_index, right_child_index) = child_indexes(index);
        tree[index] = TreeNode::Inner;
        self.left.parse_into(tree, left_child_index);
        self.right.parse_into(tree, right_child_index);
    }
}

enum InputPairElement {
    Number(u32),
    Pair(Box<InputPair>),
}

impl InputPairElement {
    fn parse_into(&self, tree: &mut [TreeNode], index: usize) {
        match &self {
            InputPairElement::Number(value) => {
                tree[index] = TreeNode::Leaf(*value);
            }
            InputPairElement::Pair(pair) => {
                pair.parse_into(tree, index);
            }
        }
    }
}

impl<'a> ParseYolo<'a> for InputPairElement {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Self {
        if stream.peek() == b'[' {
            Self::Pair(Box::new(stream.parse_yolo()))
        } else {
            Self::Number(stream.parse_yolo())
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

fn add(tree: &mut [TreeNode], other: &InputPair) {
    other.parse_into(tree, 48);
    tree[32] = TreeNode::Inner;
    for i in (1..64).step_by(2) {
        if matches!(tree[i], TreeNode::Leaf(_)) {
            explode(tree, parent_index(i));
        }
    }
    loop {
        let mut done = true;
        for i in 0..64 {
            if let TreeNode::Leaf(value) = tree[i] {
                if value >= 10 {
                    split_and_explode(tree, i);
                    done = false;
                    break;
                }
            }
        }
        if done {
            break;
        }
    }
    for i in 1..32 {
        tree[i] = tree[i << 1];
    }
    for i in 32..64 {
        tree[i] = TreeNode::Nothing;
    }
}

fn explode(tree: &mut [TreeNode], index: usize) {
    let (left_child, right_child) = child_indexes(index);
    let left_value = tree[left_child].leaf_value();
    let right_value = tree[right_child].leaf_value();
    for i in (0..index - 2).rev() {
        if let TreeNode::Leaf(left_neighbor) = &mut tree[i] {
            *left_neighbor += left_value;
            break;
        }
    }
    for i in (index + 2)..64 {
        if let TreeNode::Leaf(right_neighbor) = &mut tree[i] {
            *right_neighbor += right_value;
            break;
        }
    }
    tree[index] = TreeNode::Leaf(0);
    tree[left_child] = TreeNode::Nothing;
    tree[right_child] = TreeNode::Nothing;
}

fn split_and_explode(tree: &mut [TreeNode], index: usize) {
    let value = tree[index].leaf_value();
    let left_value = value / 2;
    let right_value = value - left_value;
    let (left_child, right_child) = child_indexes(index);
    tree[left_child] = TreeNode::Leaf(left_value);
    tree[right_child] = TreeNode::Leaf(right_value);
    tree[index] = TreeNode::Inner;
    if (left_child & 1) == 1 {
        explode(tree, index);
    }
}

fn magnitude(tree: &[TreeNode], index: usize) -> u64 {
    match tree[index] {
        TreeNode::Inner => {
            let (left_child, right_child) = child_indexes(index);
            3 * magnitude(tree, left_child) + 2 * magnitude(tree, right_child)
        }
        TreeNode::Leaf(value) => value as u64,
        TreeNode::Nothing => panic!("Should not happen"),
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