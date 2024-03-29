use derive_more::From;

pub use inventory::submit;

/// Type for the inventory value.
#[derive(Debug, Clone, Copy, From, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Value(pub u8);

inventory::collect!(Value);

#[macro_export]
macro_rules! book_values {
    ($($t:expr),*) => {
        $(
            $crate::descriptor::inventory::submit!(
                $crate::descriptor::inventory::Value($t)
            );
            $crate::descriptor::inventory::submit!(
                $crate::descriptor::inventory::Value($t | 0b1000_0000)
            );
        )*
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_book_values() {
        let values = inventory::iter::<Value>().collect::<Vec<_>>();
        let mut uniq = HashSet::new();

        for value in values {
            assert!(uniq.insert(value));
        }
    }
}
