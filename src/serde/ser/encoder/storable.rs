use crate::serde::{
    error::Error, ser::encoder::single_item::SingleItemEncoder,
};

trait Storable {
    // fn to_storable(&self) -> Vec<u8>;

    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error>;
}
