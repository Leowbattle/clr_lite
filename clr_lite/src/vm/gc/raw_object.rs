use crate::vm::interpreter::*;
use crate::vm::reflection::*;
use crate::vm::*;

///! The data for fields of an object or value type
// TODO (urgent) Make Eq compare actual value rather than pointer, which is wrong but temporary.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RawObject(pub(crate) *mut u8);

impl RawObject {
	pub fn new(_t: Type, data: *mut u8) -> RawObject {
		RawObject(data)
	}

	pub fn set(&mut self, field: Field, value: Value) {
		unsafe {
			match value {
				Value::I8(x) => *(self.0.offset(field.offset() as isize) as *mut i8) = x,
				Value::U8(x) => *(self.0.offset(field.offset() as isize) as *mut u8) = x,
				Value::I16(x) => *(self.0.offset(field.offset() as isize) as *mut i16) = x,
				Value::U16(x) => *(self.0.offset(field.offset() as isize) as *mut u16) = x,
				Value::I32(x) => *(self.0.offset(field.offset() as isize) as *mut i32) = x,
				Value::U32(x) => *(self.0.offset(field.offset() as isize) as *mut u32) = x,
				Value::I64(x) => *(self.0.offset(field.offset() as isize) as *mut i64) = x,
				Value::U64(x) => *(self.0.offset(field.offset() as isize) as *mut u64) = x,
				Value::F32(x) => *(self.0.offset(field.offset() as isize) as *mut f32) = x,
				Value::F64(x) => *(self.0.offset(field.offset() as isize) as *mut f64) = x,
				_ => unimplemented!(),
			}
		}
	}

	pub fn get(&mut self, field: Field, clr: &ClrLite) -> Value {
		let t = field.field_type().unwrap();
		let internal = clr.internal();
		let p = internal.primitives();
		unsafe {
			if t == p.sbyte {
				Value::I8(*(self.0.offset(field.offset() as isize) as *const i8))
			} else if t == p.byte {
				Value::U8(*(self.0.offset(field.offset() as isize) as *const u8))
			} else if t == p.short {
				Value::I16(*(self.0.offset(field.offset() as isize) as *const i16))
			} else if t == p.ushort {
				Value::U16(*(self.0.offset(field.offset() as isize) as *const u16))
			} else if t == p.int {
				Value::I32(*(self.0.offset(field.offset() as isize) as *const i32))
			} else if t == p.uint {
				Value::U32(*(self.0.offset(field.offset() as isize) as *const u32))
			} else if t == p.long {
				Value::I64(*(self.0.offset(field.offset() as isize) as *const i64))
			} else if t == p.ulong {
				Value::U64(*(self.0.offset(field.offset() as isize) as *const u64))
			} else if t == p.float {
				Value::F32(*(self.0.offset(field.offset() as isize) as *const f32))
			} else if t == p.double {
				Value::F64(*(self.0.offset(field.offset() as isize) as *const f64))
			} else {
				unimplemented!()
			}
		}
	}
}
