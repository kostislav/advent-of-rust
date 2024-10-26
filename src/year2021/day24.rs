use std::cmp::min;
use std::fmt::{Display, Formatter, Write};
use derive_new::new;
use crate::input::{InputData, ParseStream, ParseYolo};

pub fn part_1(input: &InputData) -> String {
    let mut registers = [Expression::Constant(0), Expression::Constant(0), Expression::Constant(0), Expression::Constant(0)];
    let mut digit = 0;

    for instruction in input.lines_as::<Instruction>() {
        match instruction {
            Instruction::Inp(register) => {
                // if digit == 4 {
                //     break;
                // }
                registers[register.index()] = Expression::InputDigit(digit);
                digit += 1;
            },
            Instruction::Binary(op, register, register_or_constant) => {
                let second_operand = match register_or_constant {
                    RegisterOrConstant::Register(register_operand) => Box::new(registers[register_operand.index()].clone()),
                    RegisterOrConstant::Constant(constant_operand) => Box::new(Expression::Constant(constant_operand)),
                };
                registers[register.index()] = evaluate(op, &registers[register.index()], &second_operand)
            }
        }
        // println!("x: {} y: {} z: {} w: {}", registers[0], registers[1], registers[2], registers[3]);  // TODO
    }
    println!("{}", registers[2]);  // TODO
    "".to_owned()
}

pub fn part_2(input: &InputData) -> String {
    "".to_owned()
}

fn evaluate(op: BinaryOperation, op1: &Expression, op2: &Expression) -> Expression {
    if let (&Expression::Constant(const_1), &Expression::Constant(const_2)) = (op1, op2) {
        let result = match op {
            BinaryOperation::Add => const_1 + const_2,
            BinaryOperation::Mul => const_1 * const_2,
            BinaryOperation::Div => const_1 / const_2, // TODO check rounding
            BinaryOperation::Mod => const_1 % const_2,
            BinaryOperation::Eql => (const_1 == const_2) as i64,
        };
        Expression::Constant(result)
    } else {
        if op == BinaryOperation::Add && op1 == &Expression::Constant(0) {
            op2.clone()
        } else if op == BinaryOperation::Mul && op2 == &Expression::Constant(1) {
            op1.clone()
        } else if op == BinaryOperation::Div && op2 == &Expression::Constant(1) {
            op1.clone()
        } else if op == BinaryOperation::Mul && op2 == &Expression::Constant(0) {
            Expression::Constant(0)
        } else if op == BinaryOperation::Eql && !op1.range().overlaps(&op2.range()) {
            Expression::Constant(0)
        } else if let (BinaryOperation::Div, Expression::Constant(divisor)) = (op, op2) {
            op1.div(*divisor)
        } else if let (BinaryOperation::Mod, Expression::Constant(modulus)) = (op, op2) {
            if op1.range().upper < *modulus {
                op1.clone()
            } else {
                op1.modulo(*modulus)
            }
        } else if let (BinaryOperation::Add, Expression::Binary(BinaryOperation::Add, sub_op1, sub_op2), Expression::Constant(term_2)) = (op, op1, op2) {
            if let Expression::Constant(term_1) = sub_op2.as_ref() {
                Expression::Binary(BinaryOperation::Add, sub_op1.clone(), Box::new(Expression::Constant(term_1 + term_2)))
            } else {
                Expression::Binary(op, Box::new(op1.clone()), Box::new(op2.clone()))
            }
        } else if let (BinaryOperation::Eql, Expression::Binary(BinaryOperation::Add, sub_op1, sub_op2), Expression::InputDigit(digit_2)) = (op, op1, op2) {
            if let Expression::InputDigit(digit_1) = sub_op1.as_ref() {
                println!("n{} + {} = n{}", digit_1 + 1, sub_op2, digit_2 + 1);  // TODO
                Expression::Constant(1)
            } else {
                Expression::Binary(op, Box::new(op1.clone()), Box::new(op2.clone()))
            }
        } else {
            Expression::Binary(op, Box::new(op1.clone()), Box::new(op2.clone()))
        }
    }
}


#[derive(Debug)]
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

    fn div(&self, divisor: i64) -> Self {
        match self {
            Expression::Constant(value) => Expression::Constant(value / divisor),
            Expression::InputDigit(_) => Expression::Constant(0),
            Expression::Binary(op, op1, op2) => {
                match op {
                    BinaryOperation::Add => evaluate(BinaryOperation::Add, &op1.div(divisor), &op2.div(divisor)),
                    BinaryOperation::Mul => {
                        if *op1.as_ref() == Expression::Constant(divisor) {
                            *op2.clone()
                        } else if *op2.as_ref() == Expression::Constant(divisor) {
                            *op1.clone()
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

    fn modulo(&self, modulus: i64) -> Self {
        match self {
            Expression::Constant(value) => Expression::Constant(value % modulus),
            Expression::InputDigit(_) => self.clone(),
            Expression::Binary(op, op1, op2) => {
                match op {
                    BinaryOperation::Add => {
                        let new_op1 = evaluate(BinaryOperation::Mod, op1, &Expression::Constant(modulus));
                        let new_op2 = evaluate(BinaryOperation::Mod, op2, &Expression::Constant(modulus));
                        if new_op1 == Expression::Constant(0) {
                            new_op2
                        } else if new_op2 == Expression::Constant(0) {
                            new_op1
                        } else {
                            panic!("Fuck")
                        }
                    },
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
                    BinaryOperation::Eql => self.clone(),
                }
            }
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(value) => value.fmt(f),
            Expression::InputDigit(index) => f.write_fmt(format_args!("n{}", index + 1)),
            Expression::Binary(op, op1, op2) => {
                f.write_str("(")?;
                op1.fmt(f)?;
                f.write_str(" ")?;
                f.write_str(match op {
                    BinaryOperation::Add => "+",
                    BinaryOperation::Mul => "*",
                    BinaryOperation::Div => "/",
                    BinaryOperation::Mod => "%",
                    BinaryOperation::Eql => "==",
                })?;
                f.write_str(" ")?;
                op2.fmt(f)?;
                f.write_str(")")
            },
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

        assert_eq!(result, "");
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, "");
    }

    fn data() -> InputData {
        InputData::from_string("
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 11
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 8
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 12
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 8
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 10
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 12
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -8
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 10
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 15
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 2
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 15
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 8
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -11
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 4
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 10
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 9
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -3
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 10
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 15
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 3
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -3
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 7
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -1
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 7
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -10
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 2
            mul y x
            add z y
            inp w
            mul x 0
            add x z
            mod x 26
            div z 26
            add x -16
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 2
            mul y x
            add z y
        ")
    }
}