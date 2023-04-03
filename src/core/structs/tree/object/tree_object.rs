pub trait TreeObject<T> {
    fn push(&mut self, value: T);
    fn get(&mut self, index: i32) -> Option<T>;
    fn find(&mut self, value: T) -> Option<i32>;
    fn remove_by_value(&mut self, value: T) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

pub trait TreeObjectFind<T> {
    fn find_greater_equal(&mut self, value: T) -> Option<(i32,T)>;
    fn find_less_equal(&mut self, value: T) -> Option<(i32,T)>;
}
