#[cfg(test)]
mod tests {
    use serde_storage::{
        de::decoder::StorageDecoder, ser::encoder::StorageEncoder,
    };

    use crate::schema::r#type::data_types::*;

    #[test]
    fn test_byte() {
        let value = Byte(25);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![25]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Byte>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }
}
