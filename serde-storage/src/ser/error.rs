use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    NoValueWritten,
}
