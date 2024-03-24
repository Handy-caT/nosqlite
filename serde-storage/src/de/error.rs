use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    InvalidLength,
    InvalidValue,
    InvalidUtf8,
    InvalidType,
    NotDeserializable,
}
