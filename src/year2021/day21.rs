use parse_yolo_derive::ParseYolo;
use crate::input::{DefaultIteratorExtras, InputData};

pub fn part_1(input: &InputData) -> usize {
    let mut players: [Player; 2] = input.lines_as::<PlayerInfo>()
        .map(|player_info| Player::new(player_info.starting_position))
        .collect_array();
    let mut die_iterator = (1..=100).cycle();
    for (turn_number, player_index) in (0..=1).cycle().enumerate() {
        let roll_sum = die_iterator.next().unwrap() + die_iterator.next().unwrap() + die_iterator.next().unwrap();
        players[player_index].advance(roll_sum);
        if players[player_index].score >= 1000 {
            return (turn_number + 1) * 3 * players[1 - player_index].score;
        }
    }
    unreachable!()
}

pub fn part_2(input: &InputData) -> i64 {
    0
}

#[derive(ParseYolo)]
#[pattern("Player {} starting position: {}")]
struct PlayerInfo {
    player_number: usize,
    starting_position: usize,
}

#[derive(Default, Copy, Clone)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(starting_position: usize) -> Self {
        Self {
            position: starting_position,
            score: 0,
        }
    }

    fn advance(&mut self, rolled_sum: usize) {
        self.position = (self.position + rolled_sum - 1) % 10 + 1;
        self.score += self.position;
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 739785);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> InputData {
        InputData::from_string("
            Player 1 starting position: 4
            Player 2 starting position: 8
        ")
    }
}