use crate::input::InputData;

pub fn part_1<I: InputData>(input: &I) -> u64 {
    let (horizontal_position, depth) = input.lines()
        .map(|line| {
            let parts = line.split_once(" ").unwrap();
            (parts.0.to_string(), parts.1.to_string())
        })
        .map(|(direction, amount)| (direction, amount.parse::<u64>().unwrap()))
        .fold((0, 0), |position, (direction, amount)| {
            match direction.as_str() {
                "forward" => (position.0 + amount, position.1),
                "down" => (position.0, position.1 + amount),
                "up" => (position.0, position.1 - amount),
                _ => panic!("Unexpected direction")
            }
        });
    horizontal_position * depth
}


#[cfg(test)]
mod tests {
    use crate::input::StringInputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 150);
    }

    // #[test]
    // fn part_2_works() {
    //     let result = part_2(&data());
    //
    //     assert_eq!(result, 5);
    // }

    fn data() -> StringInputData {
        StringInputData::new("
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ")
    }
}