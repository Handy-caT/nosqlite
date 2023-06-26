use std::mem;

// /// A base trait for converting a value to a byte array in big-endian order.
// pub trait ToBeBytes<T>
//     where
//         T: Sized
// {
//     /// Converts a value to a byte array in big-endian order.
//     /// # Returns
//     /// * `&[u8]` - Byte array in big-endian order.
//     fn to_be_bytes(&self) -> [u8; mem::size_of::<T>()];
// }
//
// impl ToBeBytes<u64> for u64 {
//     fn to_be_bytes(&self) -> [u8; mem::size_of::<u64>()] {
//         self.to_be_bytes()
//     }
// }