pub mod integer;
pub mod inventory;
pub mod r#type;

/// [`Descriptor`] of a type for encoding.
pub trait Descriptor<T, D: Description> {
    fn describe(value: T) -> Option<D>;
}

/// [`Description`] of a type for encoding.
pub trait Description {
    /// Get the bytes of the description.
    fn get_bytes(&self) -> Vec<u8>;

    /// Get the name of the type.
    fn get_name(&self) -> String;
}

/// Get the type name of a type.
pub fn get_type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}
