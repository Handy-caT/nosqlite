
pub trait TreeObject<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, item: T);
    fn peek(&self) -> Option<T>;
    
    fn get(&self, index: u64) -> Option<T>;
    fn find(&self, item: T) -> Option<u64>;
    fn remove(&mut self, index: u64) -> Option<T>;
    
    fn is_empty(&self) -> bool;
    fn len(&self) -> u64;
}