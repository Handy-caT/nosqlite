use crate::{
    book_values,
    descriptor::{get_type_name, Description, Descriptor},
    ser::encoder::StorableInteger,
    USIZE_SIZE,
};

/// Description of an integer for encoding.
#[derive(Default, Debug, Clone)]
pub struct IntegerDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

impl Description for IntegerDescription {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

/// Number of [`u8`] for the description of an integer.
const U8_NUMBER: u8 = 1;

/// Number of [`u16`] for the description of an integer.
const U16_NUMBER: u8 = 2;

/// Number of [`u32`] for the description of an integer.
const U32_NUMBER: u8 = 3;

/// Number of [`u64`] for the description of an integer.
const U64_NUMBER: u8 = 4;

/// Number of [`u128`] for the description of an integer.
const U128_NUMBER: u8 = 5;

/// Number of [`usize`] when size is 1 for the description of an integer.
const USIZE_U8_NUMBER: u8 = 6;

/// Number of [`usize`] when size is 2 for the description of an integer.
const USIZE_U16_NUMBER: u8 = 7;

/// Number of [`usize`] when size is 4 for the description of an integer.
const USIZE_U32_NUMBER: u8 = 8;

/// Number of [`usize`] when size is 8 for the description of an integer.
const USIZE_U64_NUMBER: u8 = 9;

/// Number of [`usize`] when size is 16 for the description of an integer.
const USIZE_U128_NUMBER: u8 = 10;

/// Number of [`i8`] for the description of an integer.
const I8_NUMBER: u8 = 11;

/// Number of [`i16`] for the description of an integer.
const I16_NUMBER: u8 = 12;

/// Number of [`i32`] for the description of an integer.
const I32_NUMBER: u8 = 13;

/// Number of [`i64`] for the description of an integer.
const I64_NUMBER: u8 = 14;

/// Number of [`i128`] for the description of an integer.
const I128_NUMBER: u8 = 15;

/// Number of [`isize`] when size is 1 for the description of an integer.
const ISIZE_I8_NUMBER: u8 = 16;

/// Number of [`isize`] when size is 2 for the description of an integer.
const ISIZE_I16_NUMBER: u8 = 17;

/// Number of [`isize`] when size is 4 for the description of an integer.
const ISIZE_I32_NUMBER: u8 = 18;

/// Number of [`isize`] when size is 8 for the description of an integer.
const ISIZE_I64_NUMBER: u8 = 19;

/// Number of [`isize`] when size is 16 for the description of an integer.
const ISIZE_I128_NUMBER: u8 = 20;

book_values!(
    U8_NUMBER,
    U16_NUMBER,
    U32_NUMBER,
    U64_NUMBER,
    U128_NUMBER,
    USIZE_U8_NUMBER,
    USIZE_U16_NUMBER,
    USIZE_U32_NUMBER,
    USIZE_U64_NUMBER,
    USIZE_U128_NUMBER,
    I8_NUMBER,
    I16_NUMBER,
    I32_NUMBER,
    I64_NUMBER,
    I128_NUMBER,
    ISIZE_I8_NUMBER,
    ISIZE_I16_NUMBER,
    ISIZE_I32_NUMBER,
    ISIZE_I64_NUMBER,
    ISIZE_I128_NUMBER
);

pub struct IntegerDescriptor<T>(std::marker::PhantomData<T>);

impl<T> IntegerDescriptor<T> {
    fn get_usize_description() -> u8 {
        match USIZE_SIZE {
            1 => USIZE_U8_NUMBER,
            2 => USIZE_U16_NUMBER,
            4 => USIZE_U32_NUMBER,
            8 => USIZE_U64_NUMBER,
            16 => USIZE_U128_NUMBER,
            _ => panic!("Unsupported usize size: {}", USIZE_SIZE),
        }
    }

    fn get_isize_description() -> u8 {
        match USIZE_SIZE {
            1 => ISIZE_I8_NUMBER,
            2 => ISIZE_I16_NUMBER,
            4 => ISIZE_I32_NUMBER,
            8 => ISIZE_I64_NUMBER,
            16 => ISIZE_I128_NUMBER,
            _ => panic!("Unsupported isize size: {}", USIZE_SIZE),
        }
    }
}

impl<T: StorableInteger> Descriptor<T, IntegerDescription>
    for IntegerDescriptor<T>
{
    fn describe() -> IntegerDescription {
        let type_name = get_type_name::<T>();

        let byte: u8 = match type_name {
            "u8" => U8_NUMBER,
            "u16" => U16_NUMBER,
            "u32" => U32_NUMBER,
            "u64" => U64_NUMBER,
            "u128" => U128_NUMBER,
            "usize" => Self::get_usize_description(),
            "i8" => I8_NUMBER,
            "i16" => I16_NUMBER,
            "i32" => I32_NUMBER,
            "i64" => I64_NUMBER,
            "i128" => I128_NUMBER,
            "isize" => Self::get_isize_description(),
            _ => unreachable!(),
        };

        IntegerDescription {
            bytes: vec![byte],
            name: type_name.to_string(),
        }
    }
}

macro_rules! impl_describable_integer {
    ($($t:ty),*) => {
        $(
            impl crate::descriptor::Describable<IntegerDescription> for $t {
                fn describe() -> IntegerDescription {
                    IntegerDescriptor::<$t>::describe()
                }
            }
        )*
    }
}

impl_describable_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

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

        let u8_desc = IntegerDescriptor::<u8>::describe();
        let u16_desc = IntegerDescriptor::<u16>::describe();
        let u32_desc = IntegerDescriptor::<u32>::describe();
        let u64_desc = IntegerDescriptor::<u64>::describe();
        let u128_desc = IntegerDescriptor::<u128>::describe();
        let usize_desc = IntegerDescriptor::<usize>::describe();
        let i8_desc = IntegerDescriptor::<i8>::describe();
        let i16_desc = IntegerDescriptor::<i16>::describe();
        let i32_desc = IntegerDescriptor::<i32>::describe();
        let i64_desc = IntegerDescriptor::<i64>::describe();
        let i128_desc = IntegerDescriptor::<i128>::describe();
        let isize_desc = IntegerDescriptor::<isize>::describe();

        assert_eq!(u8_desc.get_bytes(), vec![U8_NUMBER]);
        assert_eq!(u8_desc.get_name(), "u8");

        assert_eq!(u16_desc.get_bytes(), vec![U16_NUMBER]);
        assert_eq!(u16_desc.get_name(), "u16");

        assert_eq!(u32_desc.get_bytes(), vec![U32_NUMBER]);
        assert_eq!(u32_desc.get_name(), "u32");

        assert_eq!(u64_desc.get_bytes(), vec![U64_NUMBER]);
        assert_eq!(u64_desc.get_name(), "u64");

        assert_eq!(u128_desc.get_bytes(), vec![U128_NUMBER]);
        assert_eq!(u128_desc.get_name(), "u128");

        assert_eq!(
            usize_desc.get_bytes(),
            vec![IntegerDescriptor::<usize>::get_usize_description()]
        );
        assert_eq!(usize_desc.get_name(), "usize");

        assert_eq!(i8_desc.get_bytes(), vec![I8_NUMBER]);
        assert_eq!(i8_desc.get_name(), "i8");

        assert_eq!(i16_desc.get_bytes(), vec![I16_NUMBER]);
        assert_eq!(i16_desc.get_name(), "i16");

        assert_eq!(i32_desc.get_bytes(), vec![I32_NUMBER]);
        assert_eq!(i32_desc.get_name(), "i32");

        assert_eq!(i64_desc.get_bytes(), vec![I64_NUMBER]);
        assert_eq!(i64_desc.get_name(), "i64");

        assert_eq!(i128_desc.get_bytes(), vec![I128_NUMBER]);
        assert_eq!(i128_desc.get_name(), "i128");

        assert_eq!(
            isize_desc.get_bytes(),
            vec![IntegerDescriptor::<isize>::get_isize_description()]
        );
        assert_eq!(isize_desc.get_name(), "isize");
    }
}
