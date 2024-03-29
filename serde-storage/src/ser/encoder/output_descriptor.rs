use crate::descriptor::{
    backwards::{
        get_descriptor_bytes_by_type, get_length_by_description_bytes,
        get_type_by_description_bytes, is_array_by_description_bytes,
    },
    Description,
};
use smart_default::SmartDefault;

/// Descriptor bytes after encoding.
#[derive(SmartDefault, Debug, Clone)]
pub struct OutputDescriptor {
    /// List of descriptor of encoded values.
    descriptors: Vec<(Vec<u8>, String)>,
}

impl OutputDescriptor {
    /// Create a new [`OutputDescriptor`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Get the descriptors from the [`OutputDescriptor`].
    pub fn get_descriptors<'a>(&self) -> Vec<(Vec<u8>, String)> {
        self.descriptors.clone()
    }

    /// Get the descriptor bytes from the [`OutputDescriptor`].
    pub fn get_descriptor_bytes(&self) -> Vec<u8> {
        self.descriptors
            .iter()
            .flat_map(|(bytes, _)| bytes.clone())
            .collect()
    }

    /// Append a description to the [`OutputDescriptor`].
    pub fn append<D: Description>(&mut self, description: D) {
        self.descriptors
            .push((description.get_bytes(), description.get_name()));
    }

    /// Get the length of the descriptors.
    pub fn len(&self) -> usize {
        self.descriptors.len()
    }
}

/// Descriptor bytes.
#[derive(Debug, Clone)]
pub struct DescriptorBytes(pub Vec<u8>);

impl TryFrom<DescriptorBytes> for OutputDescriptor {
    type Error = DescriptorError;

    fn try_from(value: DescriptorBytes) -> Result<Self, Self::Error> {
        let bytes = value.0;
        let mut i = 0;
        let mut descriptors = Vec::new();

        while i < bytes.len() {
            let byte = bytes[i];
            if let Ok(res) = try_value_bytes(byte) {
                descriptors.push(res);
                i += 1;
            } else if let Ok(res) = try_array_bytes(&bytes[i..i + 5]) {
                descriptors.push(res);
                i += 5;
            } else {
                return Err(DescriptorError::InvalidDescriptor);
            }
        }

        Ok(OutputDescriptor { descriptors })
    }
}

/// Try to get description from single value.
fn try_value_bytes(value: u8) -> Result<(Vec<u8>, String), ()> {
    let value = vec![value];
    if is_array_by_description_bytes(value.as_slice()) {
        Err(())
    } else {
        let type_ = get_type_by_description_bytes(value.as_slice());
        if type_ == "unknown" {
            return Err(());
        }
        Ok((value, type_.to_string()))
    }
}

/// Try to get description from array value.
fn try_array_bytes(value: &[u8]) -> Result<(Vec<u8>, String), ()> {
    if !is_array_by_description_bytes(value) {
        return Err(());
    }
    let type_ = get_type_by_description_bytes(value);
    if type_ == "unknown" {
        return Err(());
    }
    let length = get_length_by_description_bytes(value);
    if let Some(length) = length {
        Ok((value.to_vec(), format!("array_{}_{}", type_, length)))
    } else {
        Err(())
    }
}

/// Descriptor parse error.
#[derive(Debug)]
pub enum DescriptorError {
    InvalidDescriptor,
}

#[cfg(test)]
mod test_descriptor {
    use crate::ser::encoder::output_descriptor::{
        try_array_bytes, try_value_bytes, DescriptorBytes, OutputDescriptor,
    };

    #[test]
    fn test_try_value_bytes() {
        let value = 1u8;
        let bytes = value.to_be_bytes();
        let res = try_value_bytes(bytes[0]);
        assert!(res.is_ok());

        let (bytes, type_) = res.unwrap();
        assert_eq!(bytes, vec![1]);
        assert_eq!(type_, "u8");
    }

    #[test]
    fn test_try_array_bytes() {
        let value = vec![1u8 | 0b1000_0000, 0, 0, 0, 3];
        let res = try_array_bytes(&value);
        assert!(res.is_ok());

        let (bytes, type_) = res.unwrap();
        assert_eq!(bytes, vec![1u8 | 0b1000_0000, 0, 0, 0, 3]);
        assert_eq!(type_, "array_u8_3");
    }

    #[test]
    fn test_try_array_bytes_invalid() {
        let value = vec![1u8, 0, 0, 0, 3];
        let res = try_array_bytes(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_try_array_bytes_invalid_length() {
        let value = vec![1u8 | 0b1000_0000, 0, 0, 0];
        let res = try_array_bytes(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_output_descriptor_try_from() {
        let value = vec![1u8, 2u8, 3u8, 4u8];
        let descriptor_bytes = DescriptorBytes(value);
        let res = OutputDescriptor::try_from(descriptor_bytes);
        assert!(res.is_ok());

        let descriptor = res.unwrap();
        let descriptors = descriptor.get_descriptors();
        assert_eq!(descriptors.len(), 4);
        assert_eq!(descriptors[0].1, "u8");
        assert_eq!(descriptors[1].1, "u16");
        assert_eq!(descriptors[2].1, "u32");
        assert_eq!(descriptors[3].1, "u64");
    }

    #[test]
    fn test_output_descriptor_try_from_array() {
        let value = vec![1u8, 2u8, 3u8 | 0b1000_0000, 0, 0, 0, 4u8];
        let descriptor_bytes = DescriptorBytes(value);
        let res = OutputDescriptor::try_from(descriptor_bytes);
        assert!(res.is_ok());

        let descriptor = res.unwrap();
        let descriptors = descriptor.get_descriptors();
        assert_eq!(descriptors.len(), 3);
        assert_eq!(descriptors[0].1, "u8");
        assert_eq!(descriptors[1].1, "u16");
        assert_eq!(descriptors[2].1, "array_u32_4");
    }
}

/// Descriptor types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorTypes(pub Vec<String>);

impl TryFrom<DescriptorTypes> for OutputDescriptor {
    type Error = DescriptorError;

    fn try_from(value: DescriptorTypes) -> Result<Self, Self::Error> {
        let mut descriptors = Vec::new();

        for type_ in value.0.iter() {
            let bytes = get_descriptor_bytes_by_type(type_);
            let type_ = type_.to_string();
            descriptors.push((bytes, type_));
        }

        Ok(OutputDescriptor { descriptors })
    }
}

#[cfg(test)]
mod test_descriptor_types {
    use crate::ser::encoder::output_descriptor::{
        DescriptorTypes, OutputDescriptor,
    };

    #[test]
    fn test_output_descriptor_try_from() {
        let value = vec![
            "u8".to_string(),
            "u16".to_string(),
            "u32".to_string(),
            "u64".to_string(),
        ];
        let descriptor_types = DescriptorTypes(value);
        let res = OutputDescriptor::try_from(descriptor_types);
        assert!(res.is_ok());

        let descriptor = res.unwrap();
        let descriptors = descriptor.get_descriptors();
        assert_eq!(descriptors.len(), 4);
        assert_eq!(descriptors[0].1, "u8");
        assert_eq!(descriptors[1].1, "u16");
        assert_eq!(descriptors[2].1, "u32");
        assert_eq!(descriptors[3].1, "u64");
    }

    #[test]
    fn test_output_descriptor_try_from_array() {
        let value = vec![
            "u8".to_string(),
            "u16".to_string(),
            "array_u32_4".to_string(),
        ];
        let descriptor_types = DescriptorTypes(value);
        let res = OutputDescriptor::try_from(descriptor_types);
        assert!(res.is_ok());

        let descriptor = res.unwrap();
        let descriptors = descriptor.get_descriptors();
        assert_eq!(descriptors.len(), 3);
        assert_eq!(descriptors[0].1, "u8");
        assert_eq!(descriptors[1].1, "u16");
        assert_eq!(descriptors[2].1, "array_u32_4");
    }
}
