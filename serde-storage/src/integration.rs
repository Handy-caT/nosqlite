#[cfg(test)]
mod tests {
    use crate::{de::decoder::StorageDecoder, ser::encoder::StorageEncoder};

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

    #[test]
    fn test_encode_decode_u32() {
        let value: u32 = 10025;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 39, 41]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit_u32(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_u64() {
        let value: u64 = 81675025;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 0, 0, 4, 222, 67, 17]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit_u64(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_u128() {
        let value: u128 = 78612539123123;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(
            bytes,
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 127, 104, 120, 37, 179]
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_u128(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }
}
