mod integer;

/// [`Descriptor`] of a type for encoding.
pub trait Descriptor<T, D: Description> {
    fn describe(&self, value: T) -> Option<D>;
}

/// [`Description`] of a type for encoding.
pub trait Description {
    fn get_bytes(&self) -> Vec<u8>;
}

/// Get the type name of a type.
pub fn get_type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}
