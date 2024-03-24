use ser_storage_derive::Storable;
use serde_storage::{
    descriptor::backwards::get_type_by_description_bytes, ser::StorageEncoder,
};

#[test]
fn test_storable() {
    #[derive(Storable)]
    struct Test {
        a: u32,
        b: u32,
    }

    let test = Test { a: 1, b: 2 };

    let mut encoder = StorageEncoder::new();

    let res = encoder.emit(test);
    assert!(res.is_ok());

    let bytes = encoder.output.get_bytes();
    assert_eq!(bytes, vec![0, 0, 0, 1, 0, 0, 0, 2]);

    let descriptor = encoder.descriptor.get_descriptors();
    assert_eq!(descriptor.len(), 2);
    assert_eq!(descriptor[0].1, "u32");
    assert_eq!(descriptor[1].1, "u32");
    assert_eq!(
        get_type_by_description_bytes(descriptor[0].0.as_ref()),
        "u32"
    );
    assert_eq!(
        get_type_by_description_bytes(descriptor[1].0.as_ref()),
        "u32"
    );
}
