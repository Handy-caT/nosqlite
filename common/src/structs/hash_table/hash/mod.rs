pub mod custom_hashable;

const RIGHT_SHIFT_VARIABLE: u8 = 19;
const SEED: u32 = 0b0100_1110_1101_0101_1011_0111_1010_1011;
const FIRST_PART_LEFT_SHIFT: u8 = 5;
const FIRST_PART_RIGHT_SHIFT: u8 = 11;
const SECOND_PART_LEFT_SHIFT: u8 = 13;
const SECOND_PART_RIGHT_SHIFT: u8 = 7;

/// Hash function that uses a custom algorithm
/// # Arguments
/// * a - First part of the data 16 bits
/// * b - Second part of the data 16 bits
/// # Returns
/// * [u8; 4] - Result of the transformation
fn transformation(a: &[u8], b: &[u8], iteration: usize) -> [u8; 4] {
    let first_part_base = u16::from(a[0]) << 8 | u16::from(b[0]);
    let second_part_base = u16::from(a[1]) << 8 | u16::from(b[1]);

    let value = u32::from(u16::from(a[0]) << 8 | u16::from(a[1])) << 16
        | u32::from(u16::from(b[0]) << 8 | u16::from(b[1]));
    let reversed_value = u32::from(u16::from(b[0]) << 8 | u16::from(b[1]))
        << 16
        | u32::from(u16::from(a[0]) << 8 | u16::from(a[1]));

    let mut first_part = first_part_base.wrapping_add(second_part_base);
    if iteration % 2 == 0 {
        first_part = first_part.rotate_left(u32::from(FIRST_PART_LEFT_SHIFT));
    } else {
        first_part = first_part.rotate_left(u32::from(FIRST_PART_RIGHT_SHIFT));
    }
    first_part ^= second_part_base;

    let mut second_part = second_part_base.wrapping_add(first_part_base);
    if iteration % 2 == 0 {
        second_part =
            second_part.rotate_right(u32::from(SECOND_PART_RIGHT_SHIFT));
    } else {
        second_part =
            second_part.rotate_right(u32::from(SECOND_PART_LEFT_SHIFT));
    }
    second_part ^= first_part_base;

    let mut data = u32::from(second_part) << 16 | u32::from(first_part);

    data = data.wrapping_mul(value);
    data = data.rotate_left(u32::from(RIGHT_SHIFT_VARIABLE));
    data ^= reversed_value;

    data.to_be_bytes()
}

/// Hash function that uses a custom algorithm. It uses custom transformation
/// function. The main idea is to use a sponge algorithm.
/// # Arguments
/// * data - Data to hash
/// # Returns
/// * u64 - Result of the hash
#[must_use]
pub fn hash(data: &[u8]) -> u64 {
    let len = data.len();
    let appendix = len.to_be_bytes();
    let real_data = [data, &appendix].concat();

    let real_data = real_data.chunks_exact(2);

    let mut hash: [u8; 4] = SEED.to_be_bytes();

    for (iter, i) in real_data.enumerate() {
        hash = transformation(i, &hash[2..4], iter);
    }

    let mut i = 0;
    let mut result: u64 = 0;
    let mut temp;
    while i < 4 {
        hash = transformation(&hash[0..2], &hash[2..4], i);
        temp = u16::from(hash[0]) << 8 | u16::from(hash[1]);
        result |= u64::from(temp) << (i * 16);
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::structs::hash_table::hash::hash;

    #[test]
    fn idempotent() {
        let data = [1u8];
        let hash_1 = hash(&data);
        let hash_2 = hash(&data);
    
        assert_eq!(hash_1, hash_2);
    }
}
