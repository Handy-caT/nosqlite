
pub struct IdGenerator {
    empty: Vec<u128>,
    counter: u128
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator {
            empty: Vec::<u128>::new(),
            counter: 0
        }
    }

    pub fn get_id(&mut self) -> u128 {
        if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.counter += 1;
            self.counter - 1
        }
    }

    pub fn return_id(&mut self, id: u128) {
        self.empty.push(id);
    }
}