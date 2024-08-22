use crate::array::{Array2d, Coordinate2d, VirtualArray2d};
use crate::graph::{shortest_path, SimpleMap};
use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    let cavern = Array2d::from_transformed_input(input, |c| c - b'0');

    shortest_path(
        Coordinate2d::new(0, 0),
        Coordinate2d::new(cavern.num_rows() as isize - 1, cavern.num_columns() as isize - 1),
        VecMap::new(cavern.num_rows(), cavern.num_columns()),
        |point| [point.up(), point.down(), point.left(), point.right()].into_iter()
            .filter_map(|neighbor| cavern.get(neighbor).map(move |&it| (neighbor, it as usize))),
    )
}

pub fn part_2(input: &InputData) -> usize {
    let small_cavern = Array2d::from_transformed_input(input, |c| c - b'0');
    let cavern = VirtualArray2d::new(
        small_cavern.num_rows() * 5,
        small_cavern.num_rows() * 5,
        |point| {
            let tile_offset = point.row() as usize / small_cavern.num_rows() + point.column() as usize / small_cavern.num_columns();
            let original_risk_value = small_cavern[Coordinate2d::new(point.row() % (small_cavern.num_rows() as isize), point.column() % (small_cavern.num_columns() as isize))] as usize;
            let modified_risk_value = original_risk_value + tile_offset;
            ((modified_risk_value - 1) % 9) + 1
        }
    );

    shortest_path(
        Coordinate2d::new(0, 0),
        Coordinate2d::new(cavern.num_rows() as isize - 1, cavern.num_columns() as isize - 1),
        VecMap::new(cavern.num_rows() * 5, cavern.num_columns() * 5),
        |point| [point.up(), point.down(), point.left(), point.right()].into_iter()
            .filter_map(|neighbor| cavern.get(neighbor).map(move |it| (neighbor, it))),
    )
}


struct VecMap {
    num_columns: usize,
    values: Vec<u16>,
}

impl VecMap {
    fn new(num_rows: usize, num_columns: usize) -> Self {
        Self {
            num_columns,
            values: vec![u16::MAX; num_rows * num_columns],
        }
    }

    fn index(&self, value: &Coordinate2d) -> usize {
        value.row() as usize * self.num_columns + value.column() as usize
    }
}

impl SimpleMap<Coordinate2d> for VecMap {
    fn insert(&mut self, key: Coordinate2d, value: usize) {
        let index = self.index(&key);
        self.values[index] = value as u16;
    }

    fn get(&self, key: &Coordinate2d) -> Option<usize> {
        let index = self.index(key);
        let value = self.values[index];
        if value == u16::MAX {
            None
        } else {
            Some(value as usize)
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

        assert_eq!(result, 40);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 315);
    }

    fn data() -> InputData {
        InputData::from_string("
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
        ")
    }
}