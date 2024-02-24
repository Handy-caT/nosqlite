use crate::descriptor::{
    r#type::CharDescription, Describable, DescribableArray, Description,
};

pub struct ArrayDescription<T, D> {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,

    /// Phantom data.
    phantom: std::marker::PhantomData<(T, D)>,
}

impl<D: Description, T: Describable<D>> ArrayDescription<T, D> {
    fn get_array_number() -> u8 {
        T::describe().get_bytes()[0] | 0b1000_0000
    }

    pub(crate) fn new(len: u32) -> Self {
        let mut name = "array_".to_string();
        name.push_str(&T::describe().get_name());
        name.push('_');
        name.push_str(&len.to_string());

        let num = Self::get_array_number();
        let mut bytes = vec![num];
        bytes.extend_from_slice(&len.to_be_bytes());

        Self {
            bytes,
            name,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T, D> Description for ArrayDescription<T, D> {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl DescribableArray<ArrayDescription<char, CharDescription>> for &str {
    fn describe(&self) -> ArrayDescription<char, CharDescription> {
        ArrayDescription::<char, CharDescription>::new(self.len() as u32)
    }
}
