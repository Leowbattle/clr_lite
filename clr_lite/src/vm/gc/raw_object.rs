use crate::vm::interpreter::*;
use crate::vm::reflection::*;

///! The data for fields of an object or value type
pub struct RawObject<'a>(&'a [u8]);

impl<'a> RawObject<'a> {
	pub fn new(t: Type, data: &'a [u8]) -> RawObject<'a> {
		if data.len() < t.size() {
			panic!(
				"Need {} bytes to store {}, only {} available",
				t.size(),
				t,
				data.len()
			);
		}
		RawObject(data)
	}

	pub fn set(&mut self, field: Field, value: Value) {
		// TODO Add in checks so this fails rather than crashing the program
		unsafe {
			match value {
				Value::I32(x) => {
					*(self.0[field.offset()..field.offset() + field.field_type().unwrap().size()]
						.as_ptr() as *mut i32) = x
				}
				_ => unimplemented!(),
			}
		}
	}
}
