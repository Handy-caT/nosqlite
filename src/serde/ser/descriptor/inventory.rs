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
            $crate::serde::ser::descriptor::inventory::submit!(
                $crate::serde::ser::descriptor::inventory::Value($t)
            );
        )*
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_book_values() {
        let values = inventory::iter::<Value>().collect::<Vec<_>>();
        let mut uniq = HashSet::new();

        for value in values {
            assert!(uniq.insert(value));
        }
    }
}