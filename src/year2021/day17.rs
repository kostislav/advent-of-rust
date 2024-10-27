use parse_yolo_derive::ParseYolo;

use crate::input::InputData;

pub fn part_1(input: &InputData) -> u64 {
    let target_area: TargetArea = input.stream().parse_yololo();
    ((target_area.y1) * (target_area.y1 + 1) / 2) as u64
}

pub fn part_2(input: &InputData) -> i64 {
    let target_area: TargetArea = input.stream().parse_yololo();

    let max_n = 2 - 2 * target_area.y1;
    let min_x_degenerate_case = (((1.0 + 8.0 * target_area.x1 as f64).sqrt() - 1.0) / 2.0).ceil() as i64;
    let max_x_degenerate_case = (((1.0 + 8.0 * target_area.x2 as f64).sqrt() - 1.0) / 2.0).floor() as i64;

    let mut num_initial_velocities = 0;
    for n in 1..=max_n {
        let min_v_x = if n > min_x_degenerate_case {
            min_x_degenerate_case
        } else {
            (target_area.x1 as f64 / n as f64 + (n as f64 - 1.0) / 2.0).ceil() as i64
        };
        let max_v_x = if n > max_x_degenerate_case {
            max_x_degenerate_case
        } else {
            (target_area.x2 as f64 / n as f64 + (n as f64 - 1.0) / 2.0).floor() as i64
        };
        let min_v_y = (target_area.y1 as f64 / n as f64 + (n as f64 - 1.0) / 2.0).ceil() as i64;
        let max_v_y = (target_area.y2 as f64 / n as f64 + (n as f64 - 1.0) / 2.0).floor() as i64;
        for v_x in min_v_x..=max_v_x {
            for v_y in min_v_y..=max_v_y {
                if (v_x > n && (n - 1) * v_x - (n - 1) * (n - 2) / 2 < target_area.x1) || (n - 1) * v_y - (n - 1) * (n - 2) / 2 > target_area.y2 {
                    num_initial_velocities += 1;
                }
            }
        }
    }

    num_initial_velocities
}


#[derive(ParseYolo)]
#[pattern("target area: x={}..{}, y={}..{}")]
struct TargetArea {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 45);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 112);
    }

    fn data() -> InputData {
        InputData::from_string("target area: x=20..30, y=-10..-5")
    }
}