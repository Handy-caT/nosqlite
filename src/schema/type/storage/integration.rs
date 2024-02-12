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

    #[test]
    fn test_bool() {
        let value = Bool(true);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![1]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Bool>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_short() {
        let value = Short(25);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 25]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Short>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_integer() {
        let value = Integer(1025);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 4, 1]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Integer>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_long() {
        let value = Long(109125);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 170, 69]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Long>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_ushort() {
        let value = UShort(1025);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![4, 1]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<UShort>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_uinteger() {
        let value = UInteger(109125);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 1, 170, 69]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<UInteger>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_ulong() {
        let value = ULong(109125);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 170, 69]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<ULong>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_float() {
        let value = Float(1.1231234);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![63, 143, 194, 130]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Float>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_double() {
        let value = Double(1.8967892514);

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![63, 254, 89, 63, 175, 162, 173, 90]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<Double>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }

    #[test]
    fn test_varchar() {
        let value = VarChar::<10>::new("Hello".to_string()).unwrap();

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit(value.clone());
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![72, 101, 108, 108, 111]);

        let mut decoder = StorageDecoder;

        let res = decoder.emit::<VarChar<10>>(bytes);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), value);
    }
}
