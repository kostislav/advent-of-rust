use crate::array::{Array2d, Coordinate2d};
use crate::graph::shortest_path;
use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    let cavern = Array2d::from_transformed_input(input, |c| c - b'0');

    shortest_path(
        Coordinate2d::new(0, 0),
        Coordinate2d::new(cavern.num_rows() as isize - 1, cavern.num_columns() as isize - 1),
        |point| [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            .map(move |(delta_x, delta_y)| Coordinate2d::new(point.row() + delta_x, point.column() + delta_y))
            .filter_map(|neighbor| cavern.get(neighbor).map(move |&it| (neighbor, it as usize)))
    )
}

pub fn part_2(input: &InputData) -> i64 {
    0
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

        assert_eq!(result, 0);
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