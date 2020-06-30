use crate::vm::interpreter::*;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Value {
	pub fn is_null_or_zero(&self) -> bool {
		match *self {
			Value::I8(x) => x == 0,
			Value::U8(x) => x == 0,
			Value::I16(x) => x == 0,
			Value::U16(x) => x == 0,
			Value::I32(x) => x == 0,
			Value::U32(x) => x == 0,
			Value::I64(x) => x == 0,
			Value::U64(x) => x == 0,
			Value::F32(x) => x == 0f32,
			Value::F64(x) => x == 0f64,
			_ => unimplemented!(),
		}
	}
}
