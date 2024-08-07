use num::integer::sqrt;
use parse_yolo_derive::ParseYolo;

use crate::input::InputData;

pub fn part_1(input: &InputData) -> u64 {
    let target_area: TargetArea = input.stream().parse_yolo();
    ((target_area.y1) * (target_area.y1 + 1) / 2) as u64
}

pub fn part_2(input: &InputData) -> i64 {
    let target_area: TargetArea = input.stream().parse_yolo();
    let mut num_initial_velocities = (target_area.x2 - target_area.x1 + 1) * (target_area.y2 - target_area.y1 + 1);

    let min_v_x = (sqrt(1 + 8 * target_area.x1) - 1) / 2;
    let max_v_x = (target_area.x2 + 1) / 2;
    let min_v_y = target_area.y1 / 2;
    let max_v_y = -target_area.y1;

    for initial_v_x in min_v_x..=max_v_x {
        for initial_v_y in min_v_y..=max_v_y {
            let mut x = 0;
            let mut y = 0;
            let mut v_x = initial_v_x;
            let mut v_y = initial_v_y;

            while x <= target_area.x2 && y >= target_area.y1 {
                if x >= target_area.x1 && y <= target_area.y2 {
                    num_initial_velocities += 1;
                    break;
                }
                x += v_x;
                y += v_y;
                if v_x > 0 {
                    v_x -= 1;
                }
                v_y -= 1;
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