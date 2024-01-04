use derive_more::From;

/// Represents an integer data type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Integer(pub i32);

/// Represents a floating point data type.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
pub struct Float(pub f32);

/// Represents a boolean data type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Boolean(pub bool);

/// Represents a char array data type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarChar<const N: u16> {
    /// The actual string data.
    pub value: String,
}

impl<const N: u16> VarChar<N> {
    /// Creates a new VarChar.
    ///
    /// Returns an error if the initial value is longer than the maximum length.
    pub fn new(value: String) -> Result<Self, &'static str> {
        Self::try_from(value)
    }
}

impl<const N: u16> TryFrom<String> for VarChar<N> {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > usize::from(N) {
            Err("Value exceeds maximum length")
        } else {
            Ok(Self {
                value,
            })
        }
    }
}

mod tests {
    use crate::schema::data_types::VarChar;

    #[test]
    fn test_varchar() {
        let varchar = VarChar::<10>::new("Hello".to_string()).unwrap();
        assert_eq!(varchar.value, "Hello");
    }

    #[test]
    fn test_varchar_too_long() {
        let varchar = VarChar::<10>::new("Hello World".to_string());
        assert!(varchar.is_err());
    }
}