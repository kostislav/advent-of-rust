use std::cmp::Ordering;
use tailcall::tailcall;

pub struct U8Map<V> {
    values: [V; 256],
}

impl<V: Default + Copy> U8Map<V> {
    pub fn new() -> Self {
        Self { values: [V::default(); 256] }
    }

    pub fn insert(&mut self, key: u8, value: V) {
        self.values[key as usize] = value;
    }

    pub fn get(&self, key: u8) -> V {
        self.values[key as usize]
    }
}

#[macro_export]
macro_rules! u8_map {
    ($($key:expr => $value:expr,)+) => { u8_map!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut _map = U8Map::new();
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}


pub fn median<T: Ord>(mut values: Vec<T>) -> T {
    let num_values = values.len();
    if num_values % 2 != 1 {
        panic!("Number of values must be odd, was {}", num_values);
    }
    quickselect(num_values / 2, &mut values);
    values.swap_remove(num_values / 2)
}


#[tailcall]
fn quickselect<T: Ord>(k: usize, values: &mut [T]) {
    if values.len() <= 5 {
        values.sort();
    } else {
        let mut pivot_index = values.len() / 2;
        let mut left_index = 0;
        let mut right_index = values.len() - 1;
        loop {
            let pivot = &values[pivot_index];
            while (&values[left_index]).cmp(pivot) == Ordering::Less {
                left_index += 1;
            }
            while (&values[right_index]).cmp(pivot) == Ordering::Greater {
                right_index -= 1;
            }

            if left_index < right_index {
                if left_index == pivot_index {
                    pivot_index = right_index;
                } else if right_index == pivot_index {
                    pivot_index = left_index;
                }

                values.swap(left_index, right_index);
            } else {
                return if k < right_index {
                    quickselect(k, &mut values[..right_index])
                } else {
                    quickselect(k - right_index, &mut values[right_index..])
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    mod median {
        use itertools::Itertools;
        use crate::collections::median;

        #[test]
        fn test_works_with_shuffled_vector() {
            let values = vec![3, 9, 16, 1, 17, 14, 2, 4, 10, 13, 11, 18, 19, 6, 12, 8, 15, 0, 20, 7, 5];

            let result = median(values);

            assert_eq!(result, 10);
        }

        #[test]
        fn test_works_with_sorted_vector() {
            let values = (0..21).collect_vec();

            let result = median(values);

            assert_eq!(result, 10);
        }
    }
}