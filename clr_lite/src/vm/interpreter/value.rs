use crate::vm::interpreter::*;

use std::cmp::Ordering;
use std::ops::*;

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
	Object(Object),
}

impl Value {
	#[inline(always)]
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
			_ => unimplemented!("Cannot check if {:?} is null or zero", self),
		}
	}
}

impl PartialOrd for Value {
	#[inline(always)]
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
			_ => unimplemented!("Cannot compare {:?} and {:?}", self, other),
		}
	}
}

impl Add for Value {
	type Output = Value;

	#[inline(always)]
	fn add(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a + b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a + b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a + b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a + b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a + b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a + b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a + b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a + b),
			(Value::F32(a), Value::F32(b)) => Value::F32(a + b),
			(Value::F64(a), Value::F64(b)) => Value::F64(a + b),
			_ => unimplemented!("Cannot add {:?} and {:?}", self, rhs),
		}
	}
}

impl Sub for Value {
	type Output = Value;

	#[inline(always)]
	fn sub(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a - b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a - b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a - b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a - b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a - b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a - b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a - b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a - b),
			(Value::F32(a), Value::F32(b)) => Value::F32(a - b),
			(Value::F64(a), Value::F64(b)) => Value::F64(a - b),
			_ => unimplemented!("Cannot subtract {:?} and {:?}", self, rhs),
		}
	}
}

impl Mul for Value {
	type Output = Value;

	#[inline(always)]
	fn mul(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a * b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a * b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a * b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a * b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a * b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a * b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a * b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a * b),
			(Value::F32(a), Value::F32(b)) => Value::F32(a * b),
			(Value::F64(a), Value::F64(b)) => Value::F64(a * b),
			_ => unimplemented!("Cannot multiply {:?} by {:?}", self, rhs),
		}
	}
}

impl Div for Value {
	type Output = Value;

	#[inline(always)]
	fn div(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a / b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a / b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a / b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a / b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a / b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a / b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a / b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a / b),
			(Value::F32(a), Value::F32(b)) => Value::F32(a / b),
			(Value::F64(a), Value::F64(b)) => Value::F64(a / b),
			_ => unimplemented!("Cannot divide {:?} by {:?}", self, rhs),
		}
	}
}

impl Rem for Value {
	type Output = Value;

	#[inline(always)]
	fn rem(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a % b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a % b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a % b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a % b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a % b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a % b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a % b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a % b),
			(Value::F32(a), Value::F32(b)) => Value::F32(a % b),
			(Value::F64(a), Value::F64(b)) => Value::F64(a % b),
			_ => unimplemented!("Cannot {:?} % {:?}", self, rhs),
		}
	}
}

impl Neg for Value {
	type Output = Value;

	#[inline(always)]
	fn neg(self) -> Value {
		match self {
			Value::I8(x) => Value::I8(-x),
			Value::I16(x) => Value::I16(-x),
			Value::I32(x) => Value::I32(-x),
			Value::I64(x) => Value::I64(-x),
			Value::F32(x) => Value::F32(-x),
			Value::F64(x) => Value::F64(-x),
			_ => unimplemented!("Cannot negate {:?}", self),
		}
	}
}

impl BitAnd for Value {
	type Output = Value;

	#[inline(always)]
	fn bitand(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a & b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a & b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a & b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a & b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a & b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a & b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a & b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a & b),
			_ => unimplemented!("Cannot {:?} && {:?}", self, rhs),
		}
	}
}

impl BitOr for Value {
	type Output = Value;

	#[inline(always)]
	fn bitor(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a | b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a | b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a | b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a | b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a | b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a | b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a | b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a | b),
			_ => unimplemented!("Cannot {:?} || {:?}", self, rhs),
		}
	}
}

impl BitXor for Value {
	type Output = Value;

	#[inline(always)]
	fn bitxor(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a ^ b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a ^ b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a ^ b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a ^ b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a ^ b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a ^ b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a ^ b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a ^ b),
			_ => unimplemented!("Cannot {:?} ^ {:?}", self, rhs),
		}
	}
}

impl Shl for Value {
	type Output = Value;

	#[inline(always)]
	fn shl(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a << b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a << b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a << b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a << b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a << b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a << b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a << b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a << b),
			_ => unimplemented!("Cannot {:?} << {:?}", self, rhs),
		}
	}
}

impl Shr for Value {
	type Output = Value;

	#[inline(always)]
	fn shr(self, rhs: Value) -> Value {
		match (self, rhs) {
			(Value::I8(a), Value::I8(b)) => Value::I8(a >> b),
			(Value::U8(a), Value::U8(b)) => Value::U8(a >> b),
			(Value::I16(a), Value::I16(b)) => Value::I16(a >> b),
			(Value::U16(a), Value::U16(b)) => Value::U16(a >> b),
			(Value::I32(a), Value::I32(b)) => Value::I32(a >> b),
			(Value::U32(a), Value::U32(b)) => Value::U32(a >> b),
			(Value::I64(a), Value::I64(b)) => Value::I64(a >> b),
			(Value::U64(a), Value::U64(b)) => Value::U64(a >> b),
			_ => unimplemented!("Cannot {:?} >> {:?}", self, rhs),
		}
	}
}

impl Not for Value {
	type Output = Value;

	#[inline(always)]
	fn not(self) -> Value {
		match self {
			Value::I8(x) => Value::I8(!x),
			Value::U8(x) => Value::U8(!x),
			Value::I16(x) => Value::I16(!x),
			Value::U16(x) => Value::U16(!x),
			Value::I32(x) => Value::I32(!x),
			Value::U32(x) => Value::U32(!x),
			Value::I64(x) => Value::I64(!x),
			Value::U64(x) => Value::U64(!x),
			_ => unimplemented!("Cannot !{:?}", self),
		}
	}
}
