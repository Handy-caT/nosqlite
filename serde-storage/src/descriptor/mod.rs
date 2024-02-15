pub mod array;
pub mod integer;
pub mod inventory;
mod r#struct;
pub mod r#type;
pub mod backwards;

/// [`Descriptor`] of a type for encoding.
pub trait Descriptor<T, D: Description> {
    fn describe() -> D;
}

/// [`Description`] of a type for encoding.
pub trait Description {
    /// Get the bytes of the description.
    fn get_bytes(&self) -> Vec<u8>;

    /// Get the name of the type.
    fn get_name(&self) -> String;
}

pub trait Describable<D: Description> {
    fn describe() -> D;
}

pub trait DescribableArray<D: Description> {
    fn describe(&self) -> D;
}

/// Get the type name of a type.
pub fn get_type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}
