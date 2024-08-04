use std::cmp::{max, min, Ordering};
use std::ops::{Add, Mul};
use crate::collections::U8Map;
use crate::input::InputData;
use crate::u8_map;

pub fn part_1(input: &InputData) -> u64 {
    version_sum(&mut BitIterator::new(input))
}

pub fn part_2(input: &InputData) -> u64 {
    evaluate(&mut BitIterator::new(input))
}

fn version_sum(bits: &mut BitIterator) -> u64 {
    let mut version = bits.next_int::<3>();
    let type_id = bits.next_int::<3>();
    if type_id == 4 {
        while bits.next() == 1 {
            bits.skip(4);
        }
        bits.skip(4);
    } else {
        version += process_operator_payload(bits, version_sum, Add::add);
    }
    version
}

fn evaluate(bits: &mut BitIterator) -> u64 {
    bits.skip(3);
    let type_id = bits.next_int::<3>();
    match type_id {
        0 => process_operator_payload(bits, evaluate, u64::add),
        1 => process_operator_payload(bits, evaluate, u64::mul),
        2 => process_operator_payload(bits, evaluate, min),
        3 => process_operator_payload(bits, evaluate, max),
        4 => {
            let mut value = 0;
            while bits.next() == 1 {
                value = (value << 4) | bits.next_int::<4>();
            }
            (value << 4) | bits.next_int::<4>()
        }
        5 => process_operator_payload(bits, evaluate, comparison_op(Ordering::Greater)),
        6 => process_operator_payload(bits, evaluate, comparison_op(Ordering::Less)),
        7 => process_operator_payload(bits, evaluate, comparison_op(Ordering::Equal)),
        _ => unreachable!(),
    }
}

fn process_operator_payload<M, A>(bits: &mut BitIterator, mut mapping: M, accumulator: A) -> u64
    where M: FnMut(&mut BitIterator) -> u64,
          A: Fn(u64, u64) -> u64 {
    if bits.next() == 0 {
        let num_bits = bits.next_int::<15>() as usize;
        let target_num_consumed = bits.num_consumed() + num_bits;
        let mut result = mapping(bits);
        while bits.num_consumed() < target_num_consumed {
            result = accumulator(result, mapping(bits));
        }
        result
    } else {
        let num_packets = bits.next_int::<11>();
        let mut result = mapping(bits);
        for _ in 1..num_packets {
            result = accumulator(result, mapping(bits));
        }
        result
    }
}

fn comparison_op(positive_result: Ordering) -> impl Fn(u64, u64) -> u64 {
    move |v1, v2| (v1.cmp(&v2) == positive_result) as u64
}

struct BitIterator {
    values: Vec<u8>,
    position: usize,
}

impl BitIterator {
    pub fn new(input: &InputData) -> Self {
        let hex_digit_values: U8Map<u8> = u8_map!(
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
        );
        let hex_values = input.raw();
        let mut values = vec![0u8; (hex_values.len() + 1) / 2];
        for i in 0..hex_values.len() {
            let digit_value = hex_digit_values.get(hex_values[i]);
            if (i & 1) == 0 {
                values[i >> 1] = digit_value << 4;
            } else {
                values[i >> 1] |= digit_value;
            }
        }
        Self {
            values,
            position: 0,
        }
    }

    pub fn next(&mut self) -> u64 {
        let mask = 1 << (7 - (self.position & 7));
        let bit = ((self.values[self.position >> 3] & mask) != 0) as u64;
        self.position += 1;
        bit
    }

    pub fn next_int<const N: usize>(&mut self) -> u64 {
        let num_bytes = (N + 7) >> 3;
        let num_bits = num_bytes << 3;
        let mut current = self.values[self.position >> 3] as u64;
        for i in 1..num_bytes {
            current = (current << 8) | self.peek_byte(i);
        }
        let position_in_byte = self.position & 7;
        let result = if position_in_byte <= num_bits - N {
            current >> (num_bits - N - position_in_byte)
        } else {
            ((current << 8) | self.peek_byte(num_bytes)) >> (num_bits + 8 - N - position_in_byte)
        };
        self.position += N;
        let mask = (1 << N) - 1;
        result & (mask as u64)
    }

    pub fn skip(&mut self, how_many: usize) {
        self.position += how_many;
    }

    pub fn num_consumed(&self) -> usize {
        self.position
    }

    fn peek_byte(&self, offset: usize) -> u64 {
        self.values[(self.position >> 3) + offset] as u64
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(&InputData::from_string("D2FE28")), 6);
        assert_eq!(part_1(&InputData::from_string("38006F45291200")), 9);
        assert_eq!(part_1(&InputData::from_string("EE00D40C823060")), 14);
        assert_eq!(part_1(&InputData::from_string("8A004A801A8002F478")), 16);
        assert_eq!(part_1(&InputData::from_string("620080001611562C8802118E34")), 12);
        assert_eq!(part_1(&InputData::from_string("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_1(&InputData::from_string("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(&InputData::from_string("C200B40A82")), 3);
        assert_eq!(part_2(&InputData::from_string("04005AC33890")), 54);
        assert_eq!(part_2(&InputData::from_string("880086C3E88112")), 7);
        assert_eq!(part_2(&InputData::from_string("CE00C43D881120")), 9);
        assert_eq!(part_2(&InputData::from_string("D8005AC2A8F0")), 1);
        assert_eq!(part_2(&InputData::from_string("F600BC2D8F")), 0);
        assert_eq!(part_2(&InputData::from_string("9C005AC2F8F0")), 0);
        assert_eq!(part_2(&InputData::from_string("9C0141080250320F1802104A08")), 1);
    }
}