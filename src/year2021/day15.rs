use crate::array::{Array2d, Coordinate2d};
use crate::graph::shortest_path;
use crate::input::InputData;

pub fn part_1(input: &InputData) -> usize {
    let cavern = Array2d::from_transformed_input(input, |c| c - b'0');

    shortest_path(
        Coordinate2d::new(0, 0),
        Coordinate2d::new(cavern.num_rows() as isize - 1, cavern.num_columns() as isize - 1),
        |point| [point.up(), point.down(), point.left(), point.right()].into_iter()
            .filter_map(|neighbor| cavern.get(neighbor).map(move |&it| (neighbor, it as usize))),
    )
}

pub fn part_2(input: &InputData) -> usize {
    let cavern = Array2d::from_transformed_input(input, |c| c - b'0');
    let num_virtual_rows = (cavern.num_rows() * 5) as isize;
    let num_virtual_columns = (cavern.num_rows() * 5) as isize;

    shortest_path(
        Coordinate2d::new(0, 0),
        Coordinate2d::new(num_virtual_rows - 1, num_virtual_columns - 1),
        |point| [point.up(), point.down(), point.left(), point.right()].into_iter()
            .filter_map(|neighbor|
                if neighbor.row() >= 0 && neighbor.column() >= 0 && neighbor.row() < num_virtual_rows && neighbor.column() < num_virtual_columns {
                    let tile_offset = neighbor.row() as usize / cavern.num_rows() + neighbor.column() as usize / cavern.num_columns();
                    let original_risk_value = cavern[Coordinate2d::new(neighbor.row() % (cavern.num_rows() as isize), neighbor.column() % (cavern.num_columns() as isize))] as usize;
                    let modified_risk_value = original_risk_value + tile_offset;
                    Some((neighbor, ((modified_risk_value - 1) % 9) + 1))
                } else {
                    None
                }
            )
    )
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