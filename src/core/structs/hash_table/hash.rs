use std::ptr::write;

const ADD_VARIABLE: u32 = 0b10110000110011000011110110101001;
const MULTIPLY_VARIABLE: u16 = 43603;
const LEFT_SHIFT_VARIABLE: u8 = 5;
const RIGHT_SHIFT_VARIABLE: u8 = 17;
const SEED: u32 = 0b01001100100100011011011110101001;
const XOR_VARIABLE: u32 = 0b11001100100100011011011110101001;
const OTHER_XOR_VARIABLE: u32 = 0b01110110010001011011011100101101;


 /// Hash function that uses a custom algorithm
 /// # Arguments
 /// * a - First part of the data 16 bits
 /// * b - Second part of the data 16 bits
 /// # Returns
 /// * [u8; 4] - Result of the transformation
fn transformation(a: &[u8], b: &[u8]) -> [u8; 4] {
    let mut data: u32 = (a[0] as u32) << 24 | (a[1] as u32) << 16 | (b[0] as u32) << 8 | b[1] as u32;

    data = data.wrapping_add(ADD_VARIABLE);
    data = data.rotate_left(LEFT_SHIFT_VARIABLE as u32);
    data = data ^ XOR_VARIABLE;
    data = data.wrapping_mul(MULTIPLY_VARIABLE as u32);
    data = data.rotate_right(RIGHT_SHIFT_VARIABLE as u32);

    data.to_be_bytes()
}

/// Hash function that uses a custom algorithm
/// It uses custom transformation function
/// The main idea is to use a sponge algorithm
/// # Arguments
/// * data - Data to hash
/// # Returns
/// * u64 - Result of the hash
pub fn custom_hash(data: &[u8]) -> u64 {
    let len = data.len();
    let mut appendix = len.to_be_bytes();

    let real_data = [data, &appendix].concat();
    let real_data = real_data.chunks_exact(2);

    let mut hash: [u8; 4] = SEED.to_be_bytes();

    for i in real_data {
        hash = transformation(&hash[0..2], i);
    }

    let mut i = 0;
    let mut result: u64 = 0;
    let mut temp: u16 = 0;
    while i < 4 {
        hash = transformation(&hash[0..2], &hash[2..4]);
        temp = (hash[0] as u16) << 8 | hash[1] as u16;
        result = result | (temp as u64) << (i * 16);
        i+=1;
    }
    result
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_custom_hash() {
        let data = [1u8];
        let hash = custom_hash(&data);

        assert_eq!(hash, 9409050860028515648u64);

        let data = [1u8, 2u8];
        let hash = custom_hash(&data);

        assert_eq!(hash, 400211062141848276u64);

        let data = [1u8, 2u8, 3u8];
        let hash = custom_hash(&data);

        assert_eq!(hash, 3000433741202943433u64);
    }
}