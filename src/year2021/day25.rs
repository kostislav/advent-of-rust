use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    let mut first = Seafloor::from_input(input);
    let mut second = first.empty_copy();
    let mut num_steps = 0;
    loop {
        if first.step(&mut second) {
            num_steps += 1;
            std::mem::swap(&mut first, &mut second);
        } else {
            break;
        }
    }
    num_steps + 1
}

pub fn part_2(_: &InputData) -> usize {
    0
}


#[derive(Eq, PartialEq, Copy, Clone)]
enum LocationContent {
    Nothing,
    EastCucumber,
    SouthCucumber,
}

impl LocationContent {
    fn from_char(c: u8) -> Self {
        match c {
            b'.' => Self::Nothing,
            b'>' => Self::EastCucumber,
            b'v' => Self::SouthCucumber,
            _ => panic!("Unexpected input"),
        }
    }
}


struct Seafloor {
    locations: Vec<LocationContent>,
    num_rows: usize,
    num_columns: usize,
}

impl Seafloor {
    fn from_input(input: &InputData) -> Self {
        let mut lines = input.lines().peekable();
        let num_columns = lines.peek().unwrap().len();
        let mut locations = Vec::with_capacity(num_columns * num_columns);
        for line in lines {
            for &c in line {
                locations.push(LocationContent::from_char(c));
            }
        }
        let num_rows = locations.len() / num_columns;
        Self { locations, num_rows, num_columns }
    }

    fn empty_copy(&self) -> Self {
        Self {
            locations: vec![LocationContent::Nothing; self.num_rows * self.num_columns],
            num_rows: self.num_rows,
            num_columns: self.num_columns,
        }
    }

    fn step(&self, next: &mut Self) -> bool {
        let mut moved = false;
        next.locations.fill(LocationContent::Nothing);
        for i in 0..self.num_rows {
            for j in 0..self.num_columns {
                let current = self.get(i as isize, j as isize);
                if current == LocationContent::EastCucumber && self.get(i as isize, j as isize + 1) == LocationContent::Nothing {
                    next.set(i, j + 1, LocationContent::EastCucumber);
                    moved = true;
                } else if current == LocationContent::SouthCucumber && (
                    (self.get(i as isize + 1, j as isize) == LocationContent::Nothing && self.get(i as isize + 1, j as isize - 1) != LocationContent::EastCucumber)
                        || (self.get(i as isize + 1, j as isize) == LocationContent::EastCucumber && self.get(i as isize + 1, j as isize + 1) == LocationContent::Nothing)
                ) {
                    next.set(i + 1, j, LocationContent::SouthCucumber);
                    moved = true;
                } else if current != LocationContent::Nothing {
                    next.set(i, j, current);
                }
            }
        }
        moved
    }

    fn get(&self, row: isize, column: isize) -> LocationContent {
        let actual_row = (row + self.num_rows as isize) as usize % self.num_rows;
        let actual_column = (column + self.num_columns as isize) as usize % self.num_columns;
        self.locations[actual_row * self.num_columns + actual_column]
    }

    fn set(&mut self, row: usize, column: usize, content: LocationContent) {
        let actual_row = row % self.num_rows;
        let actual_column = column % self.num_columns;
        self.locations[actual_row * self.num_columns + actual_column] = content;
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 58);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> InputData {
        InputData::from_string("
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        ")
    }
}