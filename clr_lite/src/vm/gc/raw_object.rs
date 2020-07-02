use crate::vm::interpreter::*;
use crate::vm::reflection::*;

use std::mem;

///! The data for fields of an object or value type
// TODO (urgent) Make Eq compare actual value rather than pointer, which is wrong but temporary.
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct RawObject {
	data: [u8],
}

impl RawObject {
	pub fn new<'a>(t: Type, data: &'a [u8]) -> &'a RawObject {
		debug_assert!(data.len() == t.size());
		unsafe { mem::transmute(data) }
	}

	pub fn set_field(&mut self, field: Field, value: Value) {
		let a = self.data.len();
		unsafe {
			match value {
				Value::I32(x) => {
					*(self.data.as_ptr().offset(field.offset() as isize) as *mut i32) = x
				}
				_ => unimplemented!(),
			}
		}
	}
}
