#[cfg(test)]
mod tests {
    use crate::de::decoder::StorageDecoder;
    use crate::ser::encoder::StorageEncoder;

    #[test]
    fn test_encode_decode_u8() {
        let value: u8 = 25;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![25]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit_u8(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_u16() {
        let value: u16 = 1025;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![4, 1]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit_u16(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }
}