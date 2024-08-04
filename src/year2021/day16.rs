use crate::input::InputData;

pub fn part_1(input: &InputData) -> u64 {
    version_sum(&mut BitIterator::new(input))
}

pub fn part_2(input: &InputData) -> i64 {
    0
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
        if bits.next() == 0 {
            let num_bits = bits.next_int(15) as usize;
            let target_num_consumed = bits.num_consumed() + num_bits;
            while bits.num_consumed() < target_num_consumed {
                version += version_sum(bits);
            }
        } else {
            let num_packets = bits.next_int(11);
            for _ in 0..num_packets {
                version += version_sum(bits);
            }
        }
    }
    version
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
            self.current = Self::hex_digit_to_value(self.values[self.index]);
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
        let result = part_2(&data());

        assert_eq!(result, 0);
    }

    fn data() -> InputData {
        InputData::from_string("
        ")
    }
}