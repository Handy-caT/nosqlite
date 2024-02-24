#[cfg(test)]
mod tests {
    use crate::{
        de::decoder::StorageDecoder,
        descriptor::backwards::{
            get_length_by_description_bytes, get_type_by_description_bytes,
        },
        ser::encoder::StorageEncoder,
    };

    #[test]
    fn test_encode_decode_u8() {
        let value: u8 = 25;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![25]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u8");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "u8"
        );

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

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u16");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "u16"
        );

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

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u32");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "u32"
        );

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

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u64");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "u64"
        );

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

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u128");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "u128"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_u128(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_bool() {
        let value: bool = true;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_bool(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![1]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "bool");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "bool"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_bool(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_i8() {
        let value: i8 = -25;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![231]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "i8");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "i8"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_i8(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_i16() {
        let value: i16 = -1025;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![251, 255]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "i16");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "i16"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_i16(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_i32() {
        let value: i32 = -10025;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![255, 255, 216, 215]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "i32");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "i32"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_i32(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_i64() {
        let value: i64 = -81675025;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![255, 255, 255, 255, 251, 33, 188, 239]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "i64");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "i64"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_i64(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_i128() {
        let value: i128 = -78612539123123;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(
            bytes,
            vec![
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 184, 128,
                151, 135, 218, 77
            ]
        );

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "i128");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "i128"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_i128(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_string() {
        let value = "Hello, world!";

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_str(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(
            bytes,
            vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
        );

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "array_char_13");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "char"
        );
        assert_eq!(
            get_length_by_description_bytes(descriptor[0].0.as_ref()),
            Some(13)
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_str(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_f32() {
        let value: f32 = 1.098_761_2;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_f32(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![63, 140, 164, 53]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "f32");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "f32"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_f32(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_f64() {
        let value: f64 = 1.90172468123;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_f64(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![63, 254, 109, 118, 219, 254, 15, 223]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "f64");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "f64"
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_f64(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_encode_decode_bytes() {
        let value = vec![1, 2, 3, 4];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_bytes(value.as_slice());
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![1, 2, 3, 4]);

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "array_u8_4");
        assert_eq!(
            get_type_by_description_bytes(descriptor[0].0.as_ref()),
            "u8"
        );
        assert_eq!(
            get_length_by_description_bytes(descriptor[0].0.as_ref()),
            Some(4)
        );

        let mut decoder = StorageDecoder;

        let res = decoder.emit_bytes(bytes.as_slice());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }
}
