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

pub fn part_2(input: &InputData) -> usize {
    let players: [Player; 2] = input.lines_as::<PlayerInfo>()
        .map(|player_info| Player::new(player_info.starting_position))
        .collect_array();
    let mut histogram = [0usize; 10];
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                histogram[i + j + k] += 1;
            }
        }
    }
    let mut memo = vec![[0; 2]; 88200];
    let results = bleh(players, 0, &histogram, &mut memo);
    results.into_iter().max().unwrap()
}

fn bleh(players: [Player; 2], active_player: usize, histogram: &[usize], memo: &mut [[usize; 2]]) -> [usize; 2] {
    let mut total = [0; 2];
    for sum in 3..=9 {
        if players[active_player].score + players[active_player].next_position(sum) >= 21 {
            total[active_player] += histogram[sum];
        } else {
            let mut updated_players = players.clone();
            updated_players[active_player].advance(sum);
            let offset = offset(&updated_players, 1 - active_player);
            if memo[offset] == [0, 0] {
                bleh(updated_players, 1 - active_player, histogram, memo);
            }
            let result = memo[offset];
            total[0] += histogram[sum] * result[0];
            total[1] += histogram[sum] * result[1];
        }
    }
    memo[offset(&players, active_player)] = total;
    total
}

fn offset(players: &[Player; 2], active_player: usize) -> usize {
    active_player * 44_100 + players[0].score * 2_100 + players[1].score * 100 + (players[0].position - 1) * 10 + (players[1].position - 1)
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
        self.position = self.next_position(rolled_sum);
        self.score += self.position;
    }

    fn next_position(&self, rolled_sum: usize) -> usize {
        (self.position + rolled_sum - 1) % 10 + 1
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

        assert_eq!(result, 444356092776315);
    }

    fn data() -> InputData {
        InputData::from_string("
            Player 1 starting position: 4
            Player 2 starting position: 8
        ")
    }
}