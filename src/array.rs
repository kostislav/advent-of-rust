use std::cmp::max;

pub struct Array2d<T> {
    num_rows: usize,
    num_columns: usize,
    values: Vec<T>,
}

impl<T> Array2d<T> {
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
            values
        }
    }
}