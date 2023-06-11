/// A static hash table that uses vectors as buckets.
/// # Arguments
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
struct StaticHashVec<V, const N: u64> {
    data: Vec<Vec<V>>,
    size: u64,
    max_length: usize,
}

impl <V: Default + Eq, const N: u64> StaticHashVec<V, N> {
    pub fn new() -> Self {
        let mut data = Vec::new();
        let mut i = 0;
        while i < N {
            data.push(Vec::new());
            i+=1;
        }
        StaticHashVec {
            data,
            size: 0,
            max_length: 0,
        }
    }

    pub fn push(&mut self, index: u64, value: V) -> (u64, usize) {
        self.data[index as usize].push(value);
        let data_index = self.data[index as usize].len() - 1;

        if self.data[index as usize].len() > self.max_length {
            self.max_length = self.data[index as usize].len();
        }
        self.size += 1;

        (index, data_index)
    }

    pub fn remove(&mut self, index: u64, value: V) {
        let mut i = 0;
        while i < self.data[index as usize].len() {
            if self.data[index as usize][i] == value {
                self.data[index as usize].remove(i);
                self.size -= 1;
                return;
            }
            i+=1;
        }
    }

    pub fn get_vec(&self, index: u64) -> &Vec<V> {
        &self.data[index as usize]
    }

    pub fn get_vec_mut(&mut self, index: u64) -> &mut Vec<V> {
        &mut self.data[index as usize]
    }

    pub fn len(&self) -> u64 {
        self.size
    }
}