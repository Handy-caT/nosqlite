use crate::{
    core::base::cast::usize::USIZE_SIZE,
    serde::ser::{
        descriptor::{get_type_name, Description, Descriptor},
        encoder::StorableInteger,
    },
};

/// Description of an integer for encoding.
#[derive(Default, Debug, Clone)]
pub struct IntegerDescription(Vec<u8>);

impl Description for IntegerDescription {
    fn get_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

pub struct IntegerDescriptor;

impl IntegerDescriptor {
    fn get_usize_description() -> u8 {
        match USIZE_SIZE {
            1 => 1 | 0b1000_0000,
            2 => 2 | 0b1000_0000,
            4 => 3 | 0b1000_0000,
            8 => 4 | 0b1000_0000,
            16 => 5 | 0b1000_0000,
            _ => panic!("Unsupported usize size: {}", USIZE_SIZE),
        }
    }

    fn get_isize_description() -> u8 {
        match USIZE_SIZE {
            1 => 1 | 0b1100_0000,
            2 => 2 | 0b1100_0000,
            4 => 3 | 0b1100_0000,
            8 => 4 | 0b1100_0000,
            16 => 5 | 0b1100_0000,
            _ => panic!("Unsupported isize size: {}", USIZE_SIZE),
        }
    }
}

impl<T: StorableInteger> Descriptor<T, IntegerDescription>
    for IntegerDescriptor
{
    fn describe(&self, value: T) -> Option<IntegerDescription> {
        let type_name = get_type_name::<T>();

        let byte: u8 = match type_name {
            "u8" => 1,
            "u16" => 2,
            "u32" => 3,
            "u64" => 4,
            "u128" => 5,
            "usize" => Self::get_usize_description(),
            "i8" => 1 | 0b0100_0000,
            "i16" => 2 | 0b0100_0000,
            "i32" => 3 | 0b0100_0000,
            "i64" => 4 | 0b0100_0000,
            "i128" => 5 | 0b0100_0000,
            "isize" => Self::get_isize_description(),
            _ => return None,
        };

        Some(IntegerDescription(vec![byte]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_descriptor() {
        let u8_val: u8 = 1;
        let u16_val: u16 = 1;
        let u32_val: u32 = 1;
        let u64_val: u64 = 1;
        let u128_val: u128 = 1;
        let usize_val: usize = 0;
        let i8_val: i8 = 1;
        let i16_val: i16 = 1;
        let i32_val: i32 = 1;
        let i64_val: i64 = 1;
        let i128_val: i128 = 1;
        let isize_val: isize = 0;

        let u8_desc = IntegerDescriptor.describe(u8_val).unwrap();
        let u16_desc = IntegerDescriptor.describe(u16_val).unwrap();
        let u32_desc = IntegerDescriptor.describe(u32_val).unwrap();
        let u64_desc = IntegerDescriptor.describe(u64_val).unwrap();
        let u128_desc = IntegerDescriptor.describe(u128_val).unwrap();
        let usize_desc = IntegerDescriptor.describe(usize_val).unwrap();
        let i8_desc = IntegerDescriptor.describe(i8_val).unwrap();
        let i16_desc = IntegerDescriptor.describe(i16_val).unwrap();
        let i32_desc = IntegerDescriptor.describe(i32_val).unwrap();
        let i64_desc = IntegerDescriptor.describe(i64_val).unwrap();
        let i128_desc = IntegerDescriptor.describe(i128_val).unwrap();
        let isize_desc = IntegerDescriptor.describe(isize_val).unwrap();

        assert_eq!(u8_desc.get_bytes(), vec![1]);
        assert_eq!(u16_desc.get_bytes(), vec![2]);
        assert_eq!(u32_desc.get_bytes(), vec![3]);
        assert_eq!(u64_desc.get_bytes(), vec![4]);
        assert_eq!(u128_desc.get_bytes(), vec![5]);
        assert_eq!(
            usize_desc.get_bytes(),
            vec![IntegerDescriptor::get_usize_description()]
        );
        assert_eq!(i8_desc.get_bytes(), vec![1 | 0b0100_0000]);
        assert_eq!(i16_desc.get_bytes(), vec![2 | 0b0100_0000]);
        assert_eq!(i32_desc.get_bytes(), vec![3 | 0b0100_0000]);
        assert_eq!(i64_desc.get_bytes(), vec![4 | 0b0100_0000]);
        assert_eq!(i128_desc.get_bytes(), vec![5 | 0b0100_0000]);
        assert_eq!(
            isize_desc.get_bytes(),
            vec![IntegerDescriptor::get_isize_description()]
        );
    }
}
