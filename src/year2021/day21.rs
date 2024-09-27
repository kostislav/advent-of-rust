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
    let mut memo = vec![[0; 2]; Player::NUM_POSSIBILITIES * Player::NUM_POSSIBILITIES];
    bleh(players[0], players[1], &histogram, &mut memo);
    memo[hash(&players[0], &players[1])].iter().copied().max().unwrap()
}

fn bleh(active_player: Player, inactive_player: Player, histogram: &[usize], memo: &mut [[usize; 2]]) {
    let mut total = [0; 2];
    for sum in 3..=9 {
        let num_identical_universes = histogram[sum];
        if active_player.score + active_player.next_position(sum) >= 21 {
            total[0] += num_identical_universes;
        } else {
            let mut updated_active_player = active_player;
            updated_active_player.advance(sum);
            let offset = hash(&inactive_player, &updated_active_player);
            if memo[offset] == [0, 0] {
                bleh(inactive_player, updated_active_player, histogram, memo);
            }
            let result = memo[offset];
            total[0] += num_identical_universes * result[1];
            total[1] += num_identical_universes * result[0];
        }
    }
    memo[hash(&active_player, &inactive_player)] = total;
}

fn hash(active_player: &Player, inactive_player: &Player) -> usize {
    active_player.hash() * Player::NUM_POSSIBILITIES + inactive_player.hash()
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
    const NUM_POSITIONS: usize = 10;
    const NUM_SCORES: usize = 21;
    const NUM_POSSIBILITIES: usize = Self::NUM_POSITIONS * Self::NUM_SCORES;

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

    fn hash(&self) -> usize {
        self.score * Self::NUM_POSITIONS + self.position - 1
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