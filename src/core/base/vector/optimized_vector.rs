pub const INITIAL_SIZE: usize = 16;

/// Struct for storing data in a vector.
/// It has a vector of data and vector of empty indexes.
/// If the empty vector is empty, then the data vector is extended.
/// If the empty vector is not empty, then index from the empty vector is used.
#[derive(Debug)]
pub struct OptimizedVector<T> {
    /// Vector of data.
    data: Vec<T>,

    /// Vector of empty indexes.
    empty: Vec<usize>,

    /// Flag to check if the item is empty.
    emptiness: Vec<bool>,

    /// Number of elements in the vector.
    length: usize,

    /// Maximum number of elements in the vector.
    max_length: usize,
}

impl<T: Clone> OptimizedVector<T> {
    /// Creates a new [`OptimizedVector`].
    /// # Returns
    /// * `OptimizedVector<T>`: New [`OptimizedVector`].
    pub fn new() -> OptimizedVector<T> {
        let mut vec = OptimizedVector {
            data: Vec::new(),
            empty: Vec::new(),
            emptiness: Vec::new(),
            length: 0,
            max_length: 0,
        };

        vec.data.reserve(INITIAL_SIZE);
        vec.emptiness.reserve(INITIAL_SIZE);

        vec.max_length = INITIAL_SIZE;

        vec
    }

    /// Pushes a value to the vector.
    /// # Arguments
    /// * `value` - Value to push
    /// # Returns
    /// * `usize` - Index of the pushed value
    pub fn push(&mut self, value: T) -> usize {
        let index = if self.empty.is_empty() {
            self.data.push(value);
            self.emptiness.push(false);
            self.length
        } else {
            let index = self.empty.pop().unwrap();
            self.data[index] = value;
            self.emptiness[index] = false;
            index
        };

        self.length += 1;

        index
    }

    /// Gets a value from the vector.
    /// # Arguments
    /// * `index` - Index of the value to get
    /// # Returns
    /// * `Option<T>` - Value at the index,
    /// or `None` if the index is out of bounds or the value is empty.
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.data.len() {
            return None;
        }

        if self.emptiness[index] {
            return None;
        }

        Some(self.data[index].clone())
    }

    /// Gets a mutable value from the vector.
    /// # Arguments
    /// * `index` - Index of the value to get
    /// # Returns
    /// * `Option<&mut T>` - Mutable value at the index,
    /// or `None` if the index is out of bounds or the value is empty.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.data.len() {
            return None;
        }

        if self.emptiness[index] {
            return None;
        }

        Some(&mut self.data[index])
    }

    /// Removes a value from the vector.
    /// # Arguments
    /// * `index` - Index of the value to remove.
    /// # Returns
    /// * `Option<T>` - Value at the index,
    /// or `None` if the index is out of bounds or the value is empty.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.data.len() {
            return None;
        }

        if self.emptiness[index] {
            return None;
        }

        self.emptiness[index] = true;
        self.empty.push(index);
        self.length -= 1;

        Some(self.data[index].clone())
    }

    /// Gets the data vector.
    /// # Returns
    /// * `&Vec<T>` - Data vector.
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    /// Gets the length of the vector.
    /// # Returns
    /// * `usize` - Length of the vector.
    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use crate::core::base::vector::optimized_vector::{
        OptimizedVector, INITIAL_SIZE,
    };

    #[test]
    fn test_optimized_vec_new() {
        let vec = OptimizedVector::<i32>::new();

        assert_eq!(vec.data.len(), 0);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.emptiness.len(), 0);

        assert_eq!(vec.length, 0);
        assert_eq!(vec.max_length, INITIAL_SIZE);
    }

    #[test]
    fn test_optimized_vec_push() {
        let mut vec = OptimizedVector::<i32>::new();
        let index = vec.push(1);

        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.emptiness.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.length, 1);
    }

    #[test]
    fn test_optimized_vec_get() {
        let mut vec = OptimizedVector::<i32>::new();
        let index = vec.push(1);

        assert_eq!(vec.get(index), Some(1));
        assert_eq!(vec.get(index + 1), None);
    }

    #[test]
    fn test_optimized_vec_get_mut() {
        let mut vec = OptimizedVector::<i32>::new();
        let index = vec.push(1);

        assert_eq!(vec.get_mut(index), Some(&mut 1));
        assert_eq!(vec.get_mut(index + 1), None);
    }

    #[test]
    fn test_optimized_vec_remove() {
        let mut vec = OptimizedVector::<i32>::new();
        let index = vec.push(1);

        assert_eq!(vec.remove(index), Some(1));
        assert_eq!(vec.remove(index + 1), None);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.emptiness.len(), 1);
        assert_eq!(vec.empty.len(), 1);
        assert_eq!(vec.empty[0], index);
        assert_eq!(vec.length, 0);
    }

    #[test]
    fn test_optimized_vec_push_remove() {
        let mut vec = OptimizedVector::<i32>::new();
        let index = vec.push(1);

        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.emptiness.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.length, 1);

        assert_eq!(vec.remove(index), Some(1));

        let index = vec.push(2);

        assert_eq!(index, 0);
        assert_eq!(vec.data.len(), 1);
        assert_eq!(vec.emptiness.len(), 1);
        assert_eq!(vec.empty.len(), 0);
        assert_eq!(vec.length, 1);
    }
}
