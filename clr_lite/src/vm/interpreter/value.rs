use crate::vm::interpreter::*;

use std::cmp::Ordering;

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

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
		match (self, other) {
			(Value::I8(a), Value::I8(b)) => a.partial_cmp(&b),
			(Value::U8(a), Value::U8(b)) => a.partial_cmp(&b),
			(Value::I16(a), Value::I16(b)) => a.partial_cmp(&b),
			(Value::U16(a), Value::U16(b)) => a.partial_cmp(&b),
			(Value::I32(a), Value::I32(b)) => a.partial_cmp(&b),
			(Value::U32(a), Value::U32(b)) => a.partial_cmp(&b),
			(Value::I64(a), Value::I64(b)) => a.partial_cmp(&b),
			(Value::U64(a), Value::U64(b)) => a.partial_cmp(&b),
			(Value::F32(a), Value::F32(b)) => a.partial_cmp(&b),
			(Value::F64(a), Value::F64(b)) => a.partial_cmp(&b),
			_ => unimplemented!(),
		}
	}
}
