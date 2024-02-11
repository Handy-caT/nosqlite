use crate::de::{decoder::single_item::SingleItemDecoder, error::Error};

pub trait Storable<T> {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<T, Error>;
}
