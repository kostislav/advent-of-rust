use bstr::ByteSlice;
use itertools::Itertools;

use crate::input::{DefaultIteratorExtras, InputData};

pub fn part_1(input: &InputData) -> usize {
    enhance(input, 2)
}

pub fn part_2(input: &InputData) -> usize {
    enhance(input, 50)
}

fn enhance(input: &InputData, iterations: usize) -> usize {
    let mut lines = input.lines().peekable();
    let algorithm: [u8; 512] = lines.next().unwrap().iter().copied().map(parse_pixel).collect_array();
    lines.next();

    let base_width = lines.peek().unwrap().len();
    let padding = iterations + 1;
    let final_width = base_width + 2 * padding;
    let mut dark_image = vec![0; final_width * final_width];
    let mut base_height = 0;
    for (i, line) in lines.enumerate() {
        base_height += 1;
        for (j, &c) in line.iter().enumerate() {
            dark_image[(padding + i) * final_width + padding + j] = parse_pixel(c);
        }
    }
    if base_height > base_width {
        dark_image.extend(std::iter::repeat(0).take((base_height - base_width) * final_width));
    }
    let final_height = base_height + 2 * padding;
    let mut light_image = vec![algorithm[0]; final_width * final_height];
    for i in (1..=iterations).step_by(2) {
        enhance_single(&dark_image, &mut light_image, iterations - i + 1, final_width, final_height, &algorithm);
        enhance_single(&light_image, &mut dark_image, iterations - (i + 1) + 1, final_width, final_height, &algorithm);
    }
    dark_image.iter().filter(|&&pixel| pixel == 1).count()
}

fn parse_pixel(c: u8) -> u8 {
    (c == b'#') as u8
}

fn enhance_single(previous_image: &[u8], current_image: &mut [u8], padding: usize, final_width: usize, final_height: usize, algorithm: &[u8]) {
    for j in padding..(final_height - padding) {
        let mut index = (0isize - previous_image[0] as isize) as usize;
        for k in padding..(final_width - padding) {
            let offset = j * final_width + k;
            index = ((index << 1) & 0b110110110)
                | (previous_image[offset - final_width + 1] << 6) as usize
                | (previous_image[offset + 1] << 3) as usize
                | previous_image[offset + final_width + 1] as usize;
            current_image[offset] = algorithm[index];
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

        assert_eq!(result, 35);
    }

    #[test]
    fn part_1_works_with_alternating_zero_element() {
        let data = InputData::from_string("
            #.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#...

            #..#.
            #....
            ##..#
            ..#..
            ..###
        ");

        let result = part_1(&data);

        assert_eq!(result, 24);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 3351);
    }

    fn data() -> InputData {
        InputData::from_string("
            ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

            #..#.
            #....
            ##..#
            ..#..
            ..###
        ")
    }
}