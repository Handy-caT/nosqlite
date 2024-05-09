/// Returns the type of the value by the description bytes. In case of array,
/// it returns base array type.
pub fn get_type_by_description_bytes(value: &[u8]) -> &'static str {
    match value[0] & 0b0111_1111 {
        0 => "bool",
        1 => "u8",
        2 => "u16",
        3 => "u32",
        4 => "u64",
        5 => "u128",
        6 => "usize_u8",
        7 => "usize_u16",
        8 => "usize_u32",
        9 => "usize_u64",
        10 => "usize_u128",
        11 => "i8",
        12 => "i16",
        13 => "i32",
        14 => "i64",
        15 => "i128",
        16 => "isize_i8",
        17 => "isize_i16",
        18 => "isize_i32",
        19 => "isize_i64",
        20 => "isize_i128",
        21 => "char",
        22 => "f32",
        23 => "f64",
        _ => "unknown",
    }
}

/// Returns true if the value is an array by the description bytes.
pub fn is_array_by_description_bytes(value: &[u8]) -> bool {
    value[0] & 0b1000_0000 != 0
}

/// Returns the length of the array by the description bytes.
pub fn get_length_by_description_bytes(value: &[u8]) -> Option<u32> {
    if value.len() < 5 {
        return None;
    }

    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&value[1..5]);
    Some(u32::from_be_bytes(bytes))
}

/// Returns the descriptor bytes by the type.
pub fn get_descriptor_bytes_by_type(type_: &str) -> Vec<u8> {
    if type_.starts_with("array_") {
        let parts: Vec<&str> = type_.split('_').collect();
        let base_type = parts[1];
        let length = parts[2].parse::<u32>().unwrap();
        let mut descriptor_byte =
            get_descriptor_byte_by_base_type(base_type).unwrap();
        descriptor_byte |= 0b1000_0000;

        let mut length_bytes = length.to_be_bytes().to_vec();
        length_bytes.insert(0, descriptor_byte);
        length_bytes
    } else {
        let descriptor_byte = get_descriptor_byte_by_base_type(type_).unwrap();
        vec![descriptor_byte]
    }
}

/// Returns the descriptor byte by the base type.
fn get_descriptor_byte_by_base_type(type_: &str) -> Option<u8> {
    match type_ {
        "bool" => Some(0),
        "u8" => Some(1),
        "u16" => Some(2),
        "u32" => Some(3),
        "u64" => Some(4),
        "u128" => Some(5),
        "usize_u8" => Some(6),
        "usize_u16" => Some(7),
        "usize_u32" => Some(8),
        "usize_u64" => Some(9),
        "usize_u128" => Some(10),
        "i8" => Some(11),
        "i16" => Some(12),
        "i32" => Some(13),
        "i64" => Some(14),
        "i128" => Some(15),
        "isize_i8" => Some(16),
        "isize_i16" => Some(17),
        "isize_i32" => Some(18),
        "isize_i64" => Some(19),
        "isize_i128" => Some(20),
        "char" => Some(21),
        "f32" => Some(22),
        "f64" => Some(23),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::descriptor::backwards::get_descriptor_bytes_by_type;

    #[test]
    fn test_get_descriptor_bytes_by_type() {
        let descriptor_bytes = get_descriptor_bytes_by_type("u32");
        assert_eq!(descriptor_bytes, vec![3]);

        let descriptor_bytes = get_descriptor_bytes_by_type("array_u8_3");
        assert_eq!(descriptor_bytes, vec![1 | 0b1000_0000, 0, 0, 0, 3]);
    }
}
