use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Sub};

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
        is_inside(point, self.num_rows, self.num_columns)
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

    pub fn get(&self, point: Coordinate2d) -> Option<&T> {
        if self.is_inside(&point) {
            Some(&self.values[point.row as usize * self.num_columns + point.column as usize])
        } else {
            None
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


#[derive(new, Clone, Copy, Eq, PartialEq, Hash)]
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


pub struct VirtualArray2d<F> {
    num_rows: usize,
    num_columns: usize,
    value_fetcher: F,
}

impl<T, F: Fn(Coordinate2d) -> T> VirtualArray2d<F> {
    pub fn new(num_rows: usize, num_columns: usize, value_fetcher: F) -> Self {
        Self { num_rows, num_columns, value_fetcher }
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    pub fn get(&self, point: Coordinate2d) -> Option<T> {
        if self.is_inside(&point) {
            Some((self.value_fetcher)(point))
        } else {
            None
        }
    }

    pub fn is_inside(&self, point: &Coordinate2d) -> bool {
        is_inside(point, self.num_rows, self.num_columns)
    }
}

fn is_inside(point: &Coordinate2d, num_rows: usize, num_columns: usize) -> bool {
    point.row >= 0 && (point.row as usize) < num_rows && point.column >= 0 && (point.column as usize) < num_columns
}


#[derive(Eq, PartialEq)]
pub struct Vector3d {
    pub coordinates: [i32; 3],
}

impl Vector3d {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { coordinates: [x, y, z] }
    }

    pub fn from_coordinates(coordinates: [i32; 3]) -> Self {
        Self { coordinates }
    }

    pub fn abs_diff(&self, other: &Vector3d) -> [u32; 3] {
        self.transformed(other, i32::abs_diff)
    }

    pub fn iter(&self) -> impl Iterator<Item=i32> + '_ {
        self.coordinates.iter().cloned()
    }

    pub fn manhattan_distance(&self, other: &Vector3d) -> u32 {
        self.abs_diff(other).into_iter().reduce(Add::add).unwrap()
    }

    fn transformed<T, F: Fn(i32, i32) -> T>(&self, other: &Self, f: F) -> [T; 3] {
        [
            f(self.coordinates[0], other.coordinates[0]),
            f(self.coordinates[1], other.coordinates[1]),
            f(self.coordinates[2], other.coordinates[2]),
        ]
    }
}

impl Sub for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3d {
            coordinates: self.transformed(rhs, Sub::sub),
        }
    }
}

impl Sub for Vector3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Add for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3d {
            coordinates: self.transformed(rhs, Add::add),
        }
    }
}

impl Add for Vector3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Display for Vector3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{},{}", self.coordinates[0], self.coordinates[1], self.coordinates[2]))
    }
}