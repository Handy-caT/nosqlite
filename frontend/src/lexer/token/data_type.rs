use derive_more::Display;
use std::str::FromStr;

/// Represents a data type
#[derive(Debug, Default, Display, Clone, PartialEq)]
pub enum DataType {
    /// Represents a boolean.
    Bool,

    /// Represents a byte.
    Byte,

    /// Represents a short.
    Short,

    /// Represents an integer.
    #[default]
    Integer,

    /// Represents a long.
    Long,

    /// Represents an unsigned short.
    UShort,

    /// Represents an unsigned integer.
    UInteger,

    /// Represents an unsigned long.
    ULong,

    /// Represents a float.
    Float,

    /// Represents a double.
    Double,

    /// Represents a variable character.
    VarChar(usize),
}

impl FromStr for DataType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bool" | "boolean" => Ok(DataType::Bool),
            "byte" | "uint1" => Ok(DataType::Byte),
            "short" | "int2" => Ok(DataType::Short),
            "integer" | "int4" => Ok(DataType::Integer),
            "long" | "int16" => Ok(DataType::Long),
            "ushort" | "uint2" => Ok(DataType::UShort),
            "uinteger" | "uint4" => Ok(DataType::UInteger),
            "ulong" | "uint16" => Ok(DataType::ULong),
            "float" => Ok(DataType::Float),
            "double" => Ok(DataType::Double),
            _ => {
                if s.to_lowercase().starts_with("varchar") {
                    let size = s[7..s.len()].parse().map_err(|_| ())?;
                    if size == 0 || size > 255 {
                        return Err(());
                    }
                    Ok(DataType::VarChar(size))
                } else {
                    Err(())
                }
            }
        }
    }
}

#[cfg(test)]
mod data_type_tests {
    use super::*;

    #[test]
    fn test_data_type_from_str() {
        assert_eq!("bool".parse(), Ok(DataType::Bool));
        assert_eq!("boolean".parse(), Ok(DataType::Bool));

        assert_eq!("byte".parse(), Ok(DataType::Byte));
        assert_eq!("uint1".parse(), Ok(DataType::Byte));

        assert_eq!("short".parse(), Ok(DataType::Short));
        assert_eq!("int2".parse(), Ok(DataType::Short));

        assert_eq!("integer".parse(), Ok(DataType::Integer));
        assert_eq!("int4".parse(), Ok(DataType::Integer));

        assert_eq!("long".parse(), Ok(DataType::Long));
        assert_eq!("int16".parse(), Ok(DataType::Long));

        assert_eq!("ushort".parse(), Ok(DataType::UShort));
        assert_eq!("uint2".parse(), Ok(DataType::UShort));

        assert_eq!("uinteger".parse(), Ok(DataType::UInteger));
        assert_eq!("uint4".parse(), Ok(DataType::UInteger));

        assert_eq!("ulong".parse(), Ok(DataType::ULong));
        assert_eq!("uint16".parse(), Ok(DataType::ULong));

        assert_eq!("float".parse(), Ok(DataType::Float));
        assert_eq!("double".parse(), Ok(DataType::Double));

        assert_eq!("varchar10".parse(), Ok(DataType::VarChar(10)));

        assert_eq!("".parse::<DataType>(), Err(()));
        assert_eq!("invalid".parse::<DataType>(), Err(()));
        assert_eq!("varchar".parse::<DataType>(), Err(()));
        assert_eq!("varchar0".parse::<DataType>(), Err(()));
        assert_eq!("varchar256".parse::<DataType>(), Err(()));
    }

    #[test]
    fn test_data_type_from_str_case_insensitive() {
        assert_eq!("bOoL".parse(), Ok(DataType::Bool));
        assert_eq!("bOoLeAn".parse(), Ok(DataType::Bool));
        assert_eq!("bYtE".parse(), Ok(DataType::Byte));
        assert_eq!("uInT1".parse(), Ok(DataType::Byte));
        assert_eq!("sHoRt".parse(), Ok(DataType::Short));
        assert_eq!("iNt2".parse(), Ok(DataType::Short));
        assert_eq!("iNtEgEr".parse(), Ok(DataType::Integer));
        assert_eq!("iNt4".parse(), Ok(DataType::Integer));
        assert_eq!("lOnG".parse(), Ok(DataType::Long));
        assert_eq!("iNt16".parse(), Ok(DataType::Long));
        assert_eq!("uShOrT".parse(), Ok(DataType::UShort));
        assert_eq!("uInT2".parse(), Ok(DataType::UShort));
        assert_eq!("uInTeGeR".parse(), Ok(DataType::UInteger));
        assert_eq!("uInT4".parse(), Ok(DataType::UInteger));
        assert_eq!("uLoNg".parse(), Ok(DataType::ULong));
        assert_eq!("uInT16".parse(), Ok(DataType::ULong));
        assert_eq!("fLoAt".parse(), Ok(DataType::Float));
        assert_eq!("dOuBlE".parse(), Ok(DataType::Double));
        assert_eq!("vArChAr10".parse(), Ok(DataType::VarChar(10)));
    }
}
