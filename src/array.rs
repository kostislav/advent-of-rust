use std::cmp::max;
use std::ops::{Index, IndexMut};
use derive_new::new;
use crate::input::InputData;

pub struct Array2d<T> {
    num_rows: usize,
    num_columns: usize,
    values: Vec<T>,
}

impl<T> Array2d<T> {
    pub fn from_transformed_input<F: Fn(u8) -> T>(input: &InputData, transformation: F) -> Self {
        let mut values = Vec::with_capacity(input.len());
        let mut lines = input.lines().peekable();
        let num_columns = lines.peek().unwrap().len();
        let mut num_rows = 0;
        for line in lines {
            num_rows += 1;
            for c in line {
                values.push(transformation(*c));
            }
        }
        Self { num_rows, num_columns, values }
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    pub fn rows(&self) -> impl Iterator<Item=ArraySlice<T>> {
        (0..self.num_rows).map(|row|
            ArraySlice {
                array: self,
                start: row * self.num_columns,
                length: self.num_columns,
                step: 1,
            }
        )
    }

    pub fn columns(&self) -> impl Iterator<Item=ArraySlice<T>> {
        (0..self.num_columns).map(|column|
            ArraySlice {
                array: self,
                start: column,
                length: self.num_rows,
                step: self.num_rows,
            }
        )
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
    }

    pub fn is_inside(&self, point: &Coordinate2d) -> bool {
        point.row >= 0 && (point.row as usize) < self.num_rows && point.column >= 0 && (point.column as usize) < self.num_columns
    }

    pub fn for_each_mut<F: FnMut(Coordinate2d, &mut T)>(&mut self, mut action: F) {
        for row in 0..self.num_rows {
            let row = row as isize;
            for column in 0..self.num_columns {
                let column = column as isize;
                let coordinate = Coordinate2d::new(row, column);
                action(coordinate, &mut self[coordinate]);
            }
        }
    }

    pub fn map_in_place<F: Fn(Coordinate2d, &T) -> T>(&mut self, transformation: F) {
        for row in 0..self.num_rows {
            let row = row as isize;
            for column in 0..self.num_columns {
                let column = column as isize;
                let coordinate = Coordinate2d::new(row, column);
                let new_value = transformation(coordinate, &self[coordinate]);
                self[coordinate] = new_value;
            }
        }
    }
}

impl<T: Copy> Array2d<T> {
    pub fn empty(num_rows: usize, num_columns: usize, fill_value: T) -> Self {
        Self {
            num_rows,
            num_columns,
            values: vec![fill_value; num_rows * num_columns],
        }
    }
}


#[derive(new, Clone, Copy, Eq, PartialEq)]
pub struct Coordinate2d {
    row: isize,
    column: isize,
}

impl Coordinate2d {
    pub fn up(&self) -> Self {
        Self { row: self.row - 1, column: self.column }
    }

    pub fn down(&self) -> Self {
        Self { row: self.row + 1, column: self.column }
    }

    pub fn left(&self) -> Self {
        Self { row: self.row, column: self.column - 1 }
    }

    pub fn right(&self) -> Self {
        Self { row: self.row, column: self.column + 1 }
    }

    pub fn row(&self) -> isize {
        self.row
    }

    pub fn column(&self) -> isize {
        self.column
    }
}


impl<T> Index<Coordinate2d> for Array2d<T> {
    type Output = T;

    fn index(&self, index: Coordinate2d) -> &Self::Output {
        &self.values[index.row as usize * self.num_columns + index.column as usize]
    }
}

impl<T> IndexMut<Coordinate2d> for Array2d<T> {
    fn index_mut(&mut self, index: Coordinate2d) -> &mut Self::Output {
        &mut self.values[index.row as usize * self.num_columns + index.column as usize]
    }
}

pub struct ArraySlice<'a, T> {
    array: &'a Array2d<T>,
    start: usize,
    length: usize,
    step: usize,
}

impl<'a, T> ArraySlice<'a, T> {
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        (0..self.length).map(|i| &self.array.values[self.start + i * self.step])
    }
}

impl<T, LI: IntoIterator<Item=T>> FromIterator<LI> for Array2d<T> {
    fn from_iter<I: IntoIterator<Item=LI>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let mut values: Vec<T> = Vec::with_capacity(max(iterator.size_hint().0, 4));
        let mut num_rows = 0;
        for row in iterator {
            values.extend(row.into_iter());
            num_rows += 1;
        }
        Array2d {
            num_rows,
            num_columns: values.len() / num_rows,
            values,
        }
    }
}