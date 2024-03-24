
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

pub fn is_array_by_description_bytes(value: &[u8]) -> bool {
    value[0] & 0b1000_0000 != 0
}

pub fn get_length_by_description_bytes(value: &[u8]) -> Option<u32> {
    if value.len() < 5 {
        return None;
    }

    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&value[1..5]);
    Some(u32::from_be_bytes(bytes))
}
