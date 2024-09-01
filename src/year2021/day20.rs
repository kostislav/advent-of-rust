use std::iter::Peekable;

use bstr::ByteSlice;

use crate::input::{DefaultIteratorExtras, InputData};

pub fn part_1(input: &InputData) -> usize {
    let mut lines = input.lines();
    let algorithm: [ImagePixel; 512] = lines.next().unwrap().iter().copied().map(ImagePixel::from_text_representation).collect_array();
    lines.next();

    let mut num_light_pixels = 0;
    let original_image = OriginalImageIterator::new(lines);
    let once_enhanced_image = EnhancedImageIterator::new(&algorithm, original_image);
    let mut twice_enhanced_image = EnhancedImageIterator::new(&algorithm, once_enhanced_image);

    let mut workspace = vec![ImagePixel::Dark; twice_enhanced_image.width()];

    for _ in 0..twice_enhanced_image.width() {
        twice_enhanced_image.load_next_into(&mut workspace);
        num_light_pixels += workspace.iter().take(twice_enhanced_image.width()).copied().filter(|&pixel| pixel == ImagePixel::Light).count();
    }
    num_light_pixels
}

pub fn part_2(input: &InputData) -> usize {
    0
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

trait ImageIterator {
    fn zero_element(&self) -> ImagePixel;

    fn width(&self) -> usize;

    fn load_next_into(&mut self, target: &mut [ImagePixel]);
}

struct OriginalImageIterator<I> {
    width: usize,
    lines: I,
}

impl<'a, I: Iterator<Item=&'a [u8]>> OriginalImageIterator<Peekable<I>> {
    pub fn new(lines: I) -> Self {
        let mut peekable_lines = lines.peekable();
        let width = peekable_lines.peek().unwrap().len();
        Self { width, lines: peekable_lines }
    }
}

impl<'a, I: Iterator<Item=&'a [u8]>> ImageIterator for OriginalImageIterator<I> {
    fn zero_element(&self) -> ImagePixel {
        ImagePixel::Dark
    }

    fn width(&self) -> usize {
        self.width
    }

    fn load_next_into(&mut self, target: &mut [ImagePixel]) {
        if let Some(line) = self.lines.next() {
            for (i, &c) in line.iter().enumerate() {
                target[i] = ImagePixel::from_text_representation(c);
            }
        } else {
            target.fill(self.zero_element());
        }
    }
}

struct EnhancedImageIterator<'a, I> {
    algorithm: &'a [ImagePixel; 512],
    previous: I,
    windows: Vec<ImagePixel>,
    zero_element: ImagePixel,
    window_offset: usize,
    width: usize,
}

impl<'a, I: ImageIterator> EnhancedImageIterator<'a, I> {
    pub fn new(algorithm: &'a [ImagePixel; 512], previous: I) -> Self {
        let previous_zero_element = previous.zero_element();
        let zero_element = if algorithm[0] == ImagePixel::Light {
            if previous_zero_element == ImagePixel::Light {
                ImagePixel::Dark
            } else {
                ImagePixel::Light
            }
        } else {
            ImagePixel::Dark
        };
        let width = previous.width() + 2;
        Self {
            algorithm,
            previous,
            windows: vec![previous_zero_element; width * 3],
            zero_element,
            window_offset: 0,
            width,
        }
    }
}

impl<'a, I: ImageIterator> ImageIterator for EnhancedImageIterator<'a, I> {
    fn zero_element(&self) -> ImagePixel {
        self.zero_element
    }

    fn width(&self) -> usize {
        self.width
    }

    fn load_next_into(&mut self, target: &mut [ImagePixel]) {
        let previous_zero_element = self.previous.zero_element();
        let previous_window_offset = self.window_offset;
        let current_window_offset = (self.window_offset + self.width) % self.windows.len();
        let next_window_offset = (current_window_offset + self.width) % self.windows.len();
        self.window_offset = current_window_offset;
        self.windows[next_window_offset] = previous_zero_element;
        self.previous.load_next_into(&mut self.windows[(next_window_offset + 1)..(next_window_offset + self.width - 1)]);
        self.windows[next_window_offset + self.width - 1] = previous_zero_element;

        let above = (previous_zero_element.as_bit() << 7) | (self.windows[previous_window_offset].as_bit() << 6);
        let here = (previous_zero_element.as_bit() << 4) | (self.windows[current_window_offset].as_bit() << 3);
        let below = (previous_zero_element.as_bit() << 1) | self.windows[next_window_offset].as_bit();
        let mut previous_index = above | here | below;

        for i in 0..(self.width - 1) {
            let index = (previous_index << 1) & 0b110110110
                | (self.windows[previous_window_offset + i + 1].as_bit() << 6)
                | (self.windows[current_window_offset + i + 1].as_bit() << 3)
                | self.windows[next_window_offset + i + 1].as_bit();
            previous_index = index;
            target[i] = self.algorithm[index];
        }
        target[self.width - 1] = self.algorithm[((previous_index << 1) & 0b110110110) | if previous_zero_element == ImagePixel::Light { 0b001001001 } else { 0 }];
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

        assert_eq!(result, 0);
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