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
    let algorithm: [u8; 512] = lines.next().unwrap().iter().map(|&c| (c == b'#') as u8).collect_array();
    lines.next();

    let base_width = lines.peek().unwrap().len();
    let final_width = base_width + 2 * (iterations + 1);
    let mut dark_image = vec![0; final_width * final_width];
    let padding = iterations + 1;
    // TODO height
    for (i, line) in lines.enumerate() {
        for (j, &c) in line.iter().enumerate() {
            dark_image[(padding + i) * final_width + padding + j] = (c == b'#') as u8;  // TODO dedup
        }
    }
    let mut light_image = vec![algorithm[0]; final_width * final_width];
    for i in (1..=iterations).step_by(2) {
        enhance_single(
            &dark_image,
            &mut light_image,
            iterations - i + 1,
            final_width,
            &algorithm,
        );
        enhance_single(
            &light_image,
            &mut dark_image,
            iterations - (i + 1) + 1,
            final_width,
            &algorithm,
        );
    }
    dark_image.iter().filter(|&&pixel| pixel == 1).count()
}

fn enhance_single(previous_image: &[u8], current_image: &mut [u8], padding: usize, final_width: usize, algorithm: &[u8]) {
    for j in padding..(final_width - padding) {
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