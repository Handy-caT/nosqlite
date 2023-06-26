use std::ptr::write;

const ADD_VARIABLE: u32 = 0b1101_0101_1100_1101_1011_0101_1010_1101;
const MULTIPLY_VARIABLE: u16 = 0b1011_0110_0101_0111;
const RIGHT_SHIFT_VARIABLE: u8 = 29;
const SEED: u32 = 0b0100_1110_1101_0101_1011_0111_1010_1011;
const XOR_VARIABLE: u32 = 0b1010_0100_1001_0101_1000_0101_0010_1011;
const FIRST_PART_XOR: u16 = 0b0101_1110_1000_0101;
const FIRST_PART_MULTIPLY: u16 = 0b1101_1110_0101_0100;
const FIRST_PART_LEFT_SHIFT: u8 = 11;
const SECOND_PART_XOR: u16 = 0b1010_1101_1000_0101;
const SECOND_PART_MULTIPLY: u16 = 0b1011_1011_0110_0101;
const SECOND_PART_RIGHT_SHIFT: u8 = 9;


 /// Hash function that uses a custom algorithm
 /// # Arguments
 /// * a - First part of the data 16 bits
 /// * b - Second part of the data 16 bits
 /// # Returns
 /// * [u8; 4] - Result of the transformation
fn transformation(a: &[u8], b: &[u8]) -> [u8; 4] {

     let mut first_part = (a[0] as u16) << 8 | b[0] as u16;
     first_part = first_part.wrapping_mul(FIRST_PART_MULTIPLY);
     first_part = first_part.rotate_left(FIRST_PART_LEFT_SHIFT as u32);
     first_part = first_part ^ FIRST_PART_XOR;

     let mut second_part = (a[1] as u16) << 8 | b[1] as u16;
     second_part = second_part.wrapping_mul(SECOND_PART_MULTIPLY);
     second_part = second_part.rotate_right(SECOND_PART_RIGHT_SHIFT as u32);
     second_part = second_part ^ SECOND_PART_XOR;

     let mut data: u32 = (first_part as u32) << 16 | second_part as u32;

     data = data.wrapping_add(ADD_VARIABLE);
     //data = data.wrapping_mul(MULTIPLY_VARIABLE as u32);
     data = data.rotate_right(RIGHT_SHIFT_VARIABLE as u32);
     data = data ^ XOR_VARIABLE;


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

    println!("len {}", len);

    let mut appendix = len.to_be_bytes();

    println!("appendix {:?}", appendix);

    let real_data = [data, &appendix].concat();

    println!("real_data");
    for i in real_data.iter() {
        print!("{:#010b} ", i);
    }
    println!();

    let real_data = real_data.chunks_exact(2);

    let mut hash: [u8; 4] = SEED.to_be_bytes();

    let mut k = 0;
    for i in real_data {
        hash = transformation(i,&hash[2..4]);

        println!("hash {}", k);
        for j in hash.iter() {
            print!("{:#010b} ", j);
        }
        println!();

        k+=1;
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

    // #[test]
    // fn test_custom_hash() {
    //     let data = [1u8];
    //     let hash = custom_hash(&data);
    //
    //     assert_eq!(hash, 9409050860028515648u64);
    //
    //     let data = [1u8, 2u8];
    //     let hash = custom_hash(&data);
    //
    //     assert_eq!(hash, 400211062141848276u64);
    //
    //     let data = [1u8, 2u8, 3u8];
    //     let hash = custom_hash(&data);
    //
    //     assert_eq!(hash, 3000433741202943433u64);
    // }
}