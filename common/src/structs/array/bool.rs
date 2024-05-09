/// A bit array.
#[derive(Debug, Clone, Default)]
pub struct BoolArray {
    /// Bytes of the array.
    bytes: Vec<u8>,
}

impl BoolArray {
    /// Create a new [`BoolArray`].
    #[must_use]
    pub fn new(size: usize) -> Self {
        let mut vec = vec![0; size];

        Self { bytes: vec }
    }

    /// Set a value in the [`BoolArray`].
    pub fn set(&mut self, index: usize, value: bool) {
        let byte_index = index / 8;
        let bit_index = index % 8;

        let byte = self.bytes[byte_index];

        let new_byte = if value {
            byte | (1 << bit_index)
        } else {
            byte & !(1 << bit_index)
        };

        self.bytes[byte_index] = new_byte;
    }

    /// Get a value from the [`BoolArray`].
    #[must_use]
    pub fn get(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;

        let byte = self.bytes[byte_index];

        let bit = (byte >> bit_index) & 1;

        match bit {
            1 => true,
            0 => false,
            _ => panic!("Bit is not 1 or 0: {bit}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_array() {
        let mut bool_array = BoolArray::new(8);

        bool_array.set(0, true);
        bool_array.set(1, false);
        bool_array.set(2, true);

        assert!(bool_array.get(0));
        assert!(!bool_array.get(1));
        assert!(bool_array.get(2));
    }
}
