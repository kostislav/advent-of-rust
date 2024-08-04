use std::cmp::{max, min, Ordering};
use std::ops::{Add, Mul};
use crate::input::InputData;

pub fn part_1(input: &InputData) -> u64 {
    version_sum(&mut BitIterator::new(input))
}

pub fn part_2(input: &InputData) -> u64 {
    evaluate(&mut BitIterator::new(input))
}

fn version_sum(bits: &mut BitIterator) -> u64 {
    let mut version = bits.next_3_bit_int();
    let type_id = bits.next_3_bit_int();
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
    let type_id = bits.next_3_bit_int();
    match type_id {
        0 => process_operator_payload(bits, evaluate, u64::add),
        1 => process_operator_payload(bits, evaluate, u64::mul),
        2 => process_operator_payload(bits, evaluate, min),
        3 => process_operator_payload(bits, evaluate, max),
        4 => {
            let mut value = 0;
            while bits.next() == 1 {
                value = (value << 4) | bits.next_int(4);
            }
            (value << 4) | bits.next_int(4)
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
        let num_bits = bits.next_int(15) as usize;
        let target_num_consumed = bits.num_consumed() + num_bits;
        let mut result = mapping(bits);
        while bits.num_consumed() < target_num_consumed {
            result = accumulator(result, mapping(bits));
        }
        result
    } else {
        let num_packets = bits.next_int(11);
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

struct BitIterator<'a> {
    values: &'a [u8],
    current: u8,
    index: usize,
    mask: u8,
}

impl<'a> BitIterator<'a> {
    pub fn new(input: &'a InputData) -> Self {
        let values = input.raw();
        Self {
            values,
            current: Self::hex_digit_to_value(values[0]),
            index: 0,
            mask: 8,
        }
    }

    pub fn next(&mut self) -> u64 {
        let bit = ((self.current & self.mask) != 0) as u64;
        if self.mask == 1 {
            self.mask = 8;
            self.index += 1;
            self.current = self.values.get(self.index).copied().map(Self::hex_digit_to_value).unwrap_or(0);
        } else {
            self.mask >>= 1;
        }
        bit
    }

    pub fn next_3_bit_int(&mut self) -> u64 {
        // TODO more efficient
        (self.next() << 2) | (self.next() << 1) | self.next()
    }

    pub fn next_int(&mut self, num_bits: usize) -> u64 {
        // TODO more efficient
        let mut value = 0;
        for _ in 0..num_bits {
            value = (value << 1) | self.next();
        }
        value
    }

    pub fn skip(&mut self, how_many: usize) {
        // TODO more efficient
        for _ in 0..how_many {
            self.next();
        }
    }

    pub fn num_consumed(&self) -> usize {
        self.index * 4 + match self.mask {
            8 => 0,
            4 => 1,
            2 => 2,
            _ => 3,
        }
    }

    fn hex_digit_to_value(hex_digit: u8) -> u8 {
        if hex_digit <= b'9' {
            hex_digit - b'0'
        } else {
            hex_digit - b'A' + 10
        }
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

    fn data() -> InputData {
        InputData::from_string("
        ")
    }
}