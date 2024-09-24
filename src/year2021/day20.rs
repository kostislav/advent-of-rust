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
    let algorithm: [ImagePixel; 512] = lines.next().unwrap().iter().copied().map(ImagePixel::from_text_representation).collect_array();
    lines.next();

    let base_width = lines.peek().unwrap().len();
    let final_width = base_width + 2 * (iterations + 1);
    let mut dark_image = vec![ImagePixel::Dark; final_width * final_width];
    let padding = iterations + 1;
    // TODO height
    for (i, line) in lines.enumerate() {
        for (j, &c) in line.iter().enumerate() {
            dark_image[(padding + i) * final_width + padding + j] = ImagePixel::from_text_representation(c);
        }
    }
    let light_pixel = if (algorithm[0].as_bit()) == 1 { ImagePixel::Light } else { ImagePixel::Dark };
    let mut light_image = vec![light_pixel; final_width * final_width];
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
    dark_image.iter().filter(|&&pixel| pixel == ImagePixel::Light).count()
}

fn enhance_single(previous_image: &[ImagePixel], current_image: &mut [ImagePixel], padding: usize, final_width: usize, algorithm: &[ImagePixel]) {
    for j in padding..(final_width - padding) {
        let mut index = (previous_image[(j - 1) * final_width + padding - 1].as_bit() << 7)
            | (previous_image[(j - 1) * final_width + padding].as_bit() << 6)
            | (previous_image[j * final_width + padding - 1].as_bit() << 4)
            | (previous_image[j * final_width + padding].as_bit() << 3)
            | (previous_image[(j + 1) * final_width + padding - 1].as_bit() << 1)
            | previous_image[(j + 1) * final_width + padding].as_bit();
        for k in padding..(final_width - padding) {
            index = ((index << 1) & 0b110110110)
                | (previous_image[(j - 1) * final_width + k + 1].as_bit() << 6)
                | (previous_image[j * final_width + k + 1].as_bit() << 3)
                | previous_image[(j + 1) * final_width + k + 1].as_bit();
            current_image[j * final_width + k] = algorithm[index];
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum ImagePixel {
    Light,
    Dark,
}

impl ImagePixel {
    fn from_text_representation(c: u8) -> Self {
        if c == b'#' {
            Self::Light
        } else {
            Self::Dark
        }
    }

    fn as_bit(&self) -> usize {
        match self {
            Self::Light => 1,
            Self::Dark => 0,
        }
    }
}

impl Default for ImagePixel {
    fn default() -> Self {
        Self::Dark
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