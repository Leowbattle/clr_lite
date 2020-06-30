use crate::vm::interpreter::*;
use crate::vm::reflection::*;

///! The data for fields of an object or value type
// TODO (urgent) Make Eq compare actual value rather than pointer, which is wrong but temporary.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RawObject(*mut u8);

impl RawObject {
	pub fn new(_t: Type, data: *mut u8) -> RawObject {
		RawObject(data)
	}

	pub fn set(&mut self, field: Field, value: Value) {
		unsafe {
			match value {
				Value::I32(x) => *(self.0.offset(field.offset() as isize) as *mut i32) = x,
				_ => unimplemented!(),
			}
		}
	}
}
