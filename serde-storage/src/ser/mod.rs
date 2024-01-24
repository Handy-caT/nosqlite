mod descriptor;
mod encoder;
mod r#struct;

use crate::error::Error;
use serde::{ser, Serialize};
//
// pub struct StorageSerializer {
//     buffer: Vec<u8>,
// }
//
// impl<'a> ser::Serializer for &'a mut StorageSerializer {
//     type Ok = ();
//
//     type Error = Error;
//
//     type SerializeSeq = Self;
//     type SerializeTuple = Self;
//     type SerializeTupleStruct = Self;
//     type SerializeTupleVariant = Self;
//     type SerializeMap = Self;
//     type SerializeStruct = Self;
//     type SerializeStructVariant = Self;
//
//     fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
//         if v {self.buffer.push(1)} else {self.buffer.push(0)}
//         Ok(())
//     }
//
//     fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//
//     fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(&v.to_be_bytes());
//         Ok(())
//     }
//
//     fn serialize_char(self, v: char) -> Result<(), Self::Error> {
//         let mut buffer: [u8; 4] = [0; 4];
//         self.serialize_str(v.encode_utf8(&mut buffer))
//     }
//
//     fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
//         self.serialize_bytes(v.as_bytes())
//     }
//
//     fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
//         self.buffer.copy_from_slice(v);
//         Ok(())
//     }
//
//     fn serialize_none(self) -> Result<(), Self::Error> {
//         Ok(())
//     }
//
//     fn serialize_some<T>(self, value: &T) -> Result<(), Self::Error>
//         where
//             T: ?Sized + Serialize,
//     {
//         value.serialize(self)
//     }
//
//     fn serialize_unit(self) -> Result<(), Self::Error> {
//         Ok(())
//     }
//
//     fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
//         self.serialize_unit()
//     }
//
//     fn serialize_unit_variant(
//         self,
//         _name: &'static str,
//         variant_index: u32,
//         _variant: &'static str,
//     ) -> Result<(), Self::Error> {
//         self.serialize_u32(variant_index)
//     }
//
//     fn serialize_newtype_struct<T>(
//         self,
//         _name: &'static str,
//         value: &T,
//     ) -> Result<(), Self::Error>
//         where
//             T: ?Sized + Serialize,
//     {
//         value.serialize(self)
//     }
//
//     fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
//         Ok(self)
//     }
//
//     fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
//         self.serialize_seq(Some(len))
//     }
//
//     fn serialize_tuple_struct(
//         self,
//         _name: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeTupleStruct, Error> {
//         self.serialize_seq(Some(len))
//     }
// }
