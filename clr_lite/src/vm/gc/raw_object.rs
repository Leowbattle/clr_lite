use crate::vm::interpreter::*;
use crate::vm::reflection::*;

///! The data for fields of an object or value type
#[derive(Debug)]
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
