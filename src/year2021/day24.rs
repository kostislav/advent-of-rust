use std::cmp::min;
use derive_new::new;
use itertools::Itertools;
use crate::input::{InputData, ParseStream, ParseYolo};

pub fn part_1(input: &InputData) -> String {
    solve(input, |pair, result| pair.populate_with_max(result))
}

pub fn part_2(input: &InputData) -> String {
    solve(input, |pair, result| pair.populate_with_min(result))
}

fn solve<F: Fn(&LinkedPair, &mut [i64])>(input: &InputData, criterion: F) -> String {
    let mut registers = [Expression::Constant(0), Expression::Constant(0), Expression::Constant(0), Expression::Constant(0)];
    let mut digit = 0;
    let mut pairs = heapless::Vec::<LinkedPair, 7>::new();

    for instruction in input.lines_as::<Instruction>() {
        match instruction {
            Instruction::Inp(register) => {
                registers[register.index()] = Expression::InputDigit(digit);
                digit += 1;
            }
            Instruction::Binary(op, register, register_or_constant) => {
                let second_operand = match register_or_constant {
                    RegisterOrConstant::Register(register_operand) => registers[register_operand.index()].clone(),
                    RegisterOrConstant::Constant(constant_operand) => Expression::Constant(constant_operand),
                };
                let (linked_pair, evaluated) = evaluate(op, std::mem::replace(&mut registers[register.index()], Expression::Constant(0)), second_operand);
                registers[register.index()] = evaluated;
                if let Some(linked_pair) = linked_pair {
                    pairs.push(linked_pair).unwrap_or_else(|_| panic!("Already full"));
                }
            }
        }
    }

    let mut result = [0; 14];
    for pair in pairs {
        criterion(&pair, &mut result);
    }

    result.iter().map(|digit| digit.to_string()).join("")
}

fn evaluate(op: BinaryOperation, op1: Expression, op2: Expression) -> (Option<LinkedPair>, Expression) {
    match (op, op1, op2) {
        (op, Expression::Constant(const_1), Expression::Constant(const_2)) => {
            let result = match op {
                BinaryOperation::Add => const_1 + const_2,
                BinaryOperation::Mul => const_1 * const_2,
                BinaryOperation::Div => const_1 / const_2,
                BinaryOperation::Mod => const_1 % const_2,
                BinaryOperation::Eql => (const_1 == const_2) as i64,
            };
            (None, Expression::Constant(result))
        }
        (BinaryOperation::Add, Expression::Constant(0), op2) => (None, op2),
        (BinaryOperation::Mul, op1, Expression::Constant(1)) => (None, op1),
        (BinaryOperation::Div, op1, Expression::Constant(1)) => (None, op1),
        (BinaryOperation::Mul, _, Expression::Constant(0)) => (None, Expression::Constant(0)),
        (BinaryOperation::Eql, op1, op2) if !op1.range().overlaps(&op2.range()) => (None, Expression::Constant(0)),
        (BinaryOperation::Div, op1, Expression::Constant(divisor)) => (None, op1.div(divisor)),
        (BinaryOperation::Mod, op1, Expression::Constant(modulus)) => {
            if op1.range().upper < modulus {
                (None, op1)
            } else {
                (None, op1.modulo(modulus))
            }
        }
        (BinaryOperation::Add, Expression::Binary(BinaryOperation::Add, sub_op1, sub_op2), Expression::Constant(term_2)) => {
            if let Expression::Constant(term_1) = sub_op2.as_ref() {
                (None, Expression::Binary(BinaryOperation::Add, sub_op1.clone(), Box::new(Expression::Constant(term_1 + term_2))))
            } else {
                (None, Expression::Binary(BinaryOperation::Add, Box::new(Expression::Binary(BinaryOperation::Add, sub_op1, sub_op2)), Box::new(Expression::Constant(term_2))))
            }
        }
        (BinaryOperation::Eql, Expression::Binary(BinaryOperation::Add, sub_op1, sub_op2), Expression::InputDigit(digit_2)) => {
            if let (Expression::InputDigit(digit_1), Expression::Constant(offset)) = (sub_op1.as_ref(), sub_op2.as_ref()) {
                (Some(LinkedPair::new(*digit_1, digit_2, *offset)), Expression::Constant(1))
            } else {
                (None, Expression::Binary(BinaryOperation::Eql, Box::new(Expression::Binary(BinaryOperation::Add, sub_op1, sub_op2)), Box::new(Expression::InputDigit(digit_2))))
            }
        }
        (op, op1, op2) => (None, Expression::Binary(op, Box::new(op1), Box::new(op2))),
    }
}


enum Register {
    X,
    Y,
    Z,
    W,
}

impl Register {
    fn index(&self) -> usize {
        match self {
            Register::X => 0,
            Register::Y => 1,
            Register::Z => 2,
            Register::W => 3,
        }
    }
}

impl<'a> ParseYolo<'a> for Register {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Self {
        match stream.next() {
            b'x' => Self::X,
            b'y' => Self::Y,
            b'z' => Self::Z,
            b'w' => Self::W,
            _ => panic!("Parsing failed"),
        }
    }
}


enum RegisterOrConstant {
    Register(Register),
    Constant(i64),
}

impl<'a> ParseYolo<'a> for RegisterOrConstant {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Self {
        let next = stream.peek();
        if next == b'-' || (next >= b'0' && next <= b'9') {
            Self::Constant(stream.parse_yolo())
        } else {
            Self::Register(stream.parse_yolo())
        }
    }
}


#[derive(Clone, Copy, Eq, PartialEq)]
enum BinaryOperation {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl<'a> ParseYolo<'a> for BinaryOperation {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Self {
        if stream.try_consume("add") {
            Self::Add
        } else if stream.try_consume("mul") {
            Self::Mul
        } else if stream.try_consume("div") {
            Self::Div
        } else if stream.try_consume("mod") {
            Self::Mod
        } else if stream.try_consume("eql") {
            Self::Eql
        } else {
            panic!("Parsing failed")
        }
    }
}


enum Instruction {
    Inp(Register),
    Binary(BinaryOperation, Register, RegisterOrConstant),
}

impl<'a> ParseYolo<'a> for Instruction {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Self {
        if stream.try_consume("inp ") {
            Self::Inp(stream.parse_yolo())
        } else {
            let operation = stream.parse_yolo();
            stream.expect(" ");
            let register = stream.parse_yolo();
            stream.expect(" ");
            let register_or_constant = stream.parse_yolo();
            Self::Binary(operation, register, register_or_constant)
        }
    }
}


#[derive(new)]
struct Range {
    lower: i64,
    upper: i64,
}

impl Range {
    fn single(value: i64) -> Self {
        Self { lower: value, upper: value }
    }

    fn combine<F: Fn(i64, i64) -> i64>(&self, other: &Self, op: F) -> Self {
        let combinations = [op(self.lower, other.lower), op(self.lower, other.upper), op(self.upper, other.lower), op(self.upper, other.upper)];
        Range::new(*combinations.iter().min().unwrap(), *combinations.iter().max().unwrap())
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.lower <= other.upper && other.lower <= self.upper
    }
}


#[derive(Clone, Eq, PartialEq)]
enum Expression {
    Constant(i64),
    InputDigit(usize),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn range(&self) -> Range {
        match self {
            &Expression::Constant(value) => Range::single(value),
            &Expression::InputDigit(index) => Range::new(1, 9),
            Expression::Binary(op, op1, op2) => {
                let range_1 = op1.range();
                let range_2 = op2.range();
                match op {
                    BinaryOperation::Add => Range::new(range_1.lower + range_2.lower, range_1.upper + range_2.upper),
                    BinaryOperation::Mul => range_1.combine(&range_2, |i, j| i * j),
                    BinaryOperation::Div => range_1.combine(&range_2, |i, j| i / j),
                    BinaryOperation::Mod => Range::new(0, min(range_2.upper, range_1.upper)),
                    BinaryOperation::Eql => Range::new(0, 1),
                }
            }
        }
    }

    fn div(self, divisor: i64) -> Self {
        match self {
            Expression::Constant(value) => Expression::Constant(value / divisor),
            Expression::InputDigit(_) => Expression::Constant(0),
            Expression::Binary(op, op1, op2) => {
                match op {
                    BinaryOperation::Add => evaluate(BinaryOperation::Add, op1.div(divisor), op2.div(divisor)).1,
                    BinaryOperation::Mul => {
                        if *op1.as_ref() == Expression::Constant(divisor) {
                            *op2
                        } else if *op2.as_ref() == Expression::Constant(divisor) {
                            *op1
                        } else {
                            panic!("Fuck")
                        }
                    }
                    BinaryOperation::Div => panic!("Fuck"),
                    BinaryOperation::Mod => panic!("Fuck"),
                    BinaryOperation::Eql => Expression::Constant(0),
                }
            }
        }
    }

    fn modulo(self, modulus: i64) -> Self {
        match self {
            Expression::Constant(value) => Expression::Constant(value % modulus),
            Expression::InputDigit(_) => self,
            Expression::Binary(op, op1, op2) => {
                match op {
                    BinaryOperation::Add => {
                        let new_op1 = evaluate(BinaryOperation::Mod, *op1, Expression::Constant(modulus)).1;
                        let new_op2 = evaluate(BinaryOperation::Mod, *op2, Expression::Constant(modulus)).1;
                        if new_op1 == Expression::Constant(0) {
                            new_op2
                        } else if new_op2 == Expression::Constant(0) {
                            new_op1
                        } else {
                            panic!("Fuck")
                        }
                    }
                    BinaryOperation::Mul => {
                        if *op1.as_ref() == Expression::Constant(modulus) {
                            Expression::Constant(0)
                        } else if *op2.as_ref() == Expression::Constant(modulus) {
                            Expression::Constant(0)
                        } else {
                            panic!("Fuck")
                        }
                    }
                    BinaryOperation::Div => panic!("Fuck"),
                    BinaryOperation::Mod => panic!("Fuck"),
                    BinaryOperation::Eql => Expression::Binary(op, op1, op2),
                }
            }
        }
    }
}

#[derive(new)]
struct LinkedPair {
    first_digit: usize,
    second_digit: usize,
    offset: i64,
}

impl LinkedPair {
    fn populate_with_max(&self, target: &mut [i64]) {
        if self.offset < 0 {
            target[self.first_digit] = 9;
            target[self.second_digit] = 9 + self.offset;
        } else {
            target[self.first_digit] = 9 - self.offset;
            target[self.second_digit] = 9;
        }
    }

    fn populate_with_min(&self, target: &mut [i64]) {
        if self.offset < 0 {
            target[self.first_digit] = 1 - self.offset;
            target[self.second_digit] = 1;
        } else {
            target[self.first_digit] = 1;
            target[self.second_digit] = 1 + self.offset;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, "99598963999971");
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, "93151411711211");
    }

    fn data() -> InputData {
        InputData::from_file("input/year2021/day24")
    }
}