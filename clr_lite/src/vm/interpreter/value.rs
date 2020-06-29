use crate::vm::interpreter::*;

#[derive(Debug)]
pub enum Value {
	I8(i8),
	U8(u8),
	I16(i16),
	U16(u16),
	I32(i32),
	U32(u32),
	I64(i64),
	U64(u64),
	F32(f32),
	F64(f64),
	ValueType(RawObject),
}
