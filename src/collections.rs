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