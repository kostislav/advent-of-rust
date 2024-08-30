use std::iter::Peekable;
use bstr::ByteSlice;

use crate::input::{DefaultIteratorExtras, InputData, WrappingArray};

pub fn part_1(input: &InputData) -> usize {
    let mut lines = input.lines();
    let algorithm: [ImagePixel; 512] = lines.next().unwrap().iter().copied().map(ImagePixel::from_text_representation).collect_array();
    lines.next();

    let mut num_light_pixels = 0;
    let original_image = OriginalImageIterator::new(lines);
    let once_enhanced_image = EnhancedImageIterator::new(&algorithm, original_image);
    let mut twice_enhanced_image = EnhancedImageIterator::new(&algorithm, once_enhanced_image);

    while let Some(line) = twice_enhanced_image.next() {
        num_light_pixels += line.filter(|&pixel| pixel == ImagePixel::Light).count();
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

    fn next(&mut self) -> Option<impl Iterator<Item=ImagePixel>>;
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

    fn next(&mut self) -> Option<impl Iterator<Item=ImagePixel>> {
        self.lines.next().map(|line| line.iter().copied().map(ImagePixel::from_text_representation))
    }
}

struct EnhancedImageIterator<'a, I> {
    algorithm: &'a [ImagePixel; 512],
    previous: I,
    window: WrappingArray<Vec<ImagePixel>, 3>,
    zero_element: ImagePixel,
    tail: usize,
}

impl<'a, I: ImageIterator> EnhancedImageIterator<'a, I> {
    pub fn new(algorithm: &'a [ImagePixel; 512], previous: I) -> Self {
        let zero_element = if algorithm[0] == ImagePixel::Light {
            if previous.zero_element() == ImagePixel::Light {
                ImagePixel::Dark
            } else {
                ImagePixel::Light
            }
        } else {
            ImagePixel::Dark
        };
        let width = previous.width() + 4;
        let mut window = WrappingArray::new(|| Vec::with_capacity(width));
        window[0].resize(width, previous.zero_element());
        window[1].resize(width, previous.zero_element());
        Self {
            algorithm,
            previous,
            window,
            zero_element,
            tail: 0,
        }
    }

    fn index(&self, i: usize) -> usize {
        let above = (self.window[-1][i - 1].as_bit() << 8) | (self.window[-1][i].as_bit() << 7) | (self.window[-1][i + 1].as_bit() << 6);
        let here = (self.window[0][i - 1].as_bit() << 5) | (self.window[0][i].as_bit() << 4) | (self.window[0][i + 1].as_bit() << 3);
        let below = (self.window[1][i - 1].as_bit() << 2) | (self.window[1][i].as_bit() << 1) | self.window[1][i + 1].as_bit();
        above | here | below
    }
}

impl<'a, I: ImageIterator> ImageIterator for EnhancedImageIterator<'a, I> {
    fn zero_element(&self) -> ImagePixel {
        self.zero_element
    }

    fn width(&self) -> usize {
        self.previous.width() + 2
    }

    fn next(&mut self) -> Option<impl Iterator<Item=ImagePixel>> {
        let width = self.width();
        let previous_zero_element = self.previous.zero_element();
        self.window.rotate_left();
        let has_next = if let Some(line) = self.previous.next() {
            self.window[1].clear();
            self.window[1].extend_from_slice(&[previous_zero_element; 2]);
            self.window[1].extend(line);
            self.window[1].extend_from_slice(&[previous_zero_element; 2]);
            true
        } else {
            if self.tail < 2 {
                self.tail += 1;
                self.window[1].clear();
                self.window[1].resize(width + 2, previous_zero_element);
                true
            } else {
                false
            }
        };

        if has_next {
            Some((1..=width).map(|i| self.algorithm[self.index(i)]))
        } else {
            None
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