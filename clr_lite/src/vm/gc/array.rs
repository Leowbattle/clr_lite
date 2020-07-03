use crate::vm::gc::*;
use crate::vm::interpreter::*;
use crate::vm::reflection::*;

use std::convert::TryInto;
use std::slice;

#[derive(Debug)]
pub(crate) struct ArrayHeader {
	pub object_header: ObjectHeader,
	pub element_type: Type,
	pub length: usize,
}

// When I was writing this I initially had `data: [Value]`, but that would make it so each element
// takes 16 bytes, even if it is a `byte[]`. Also that data structure would technically be able to
// hold elements of different types, e.g.: [Value::I32(4), Value::F64(3.14159)]

#[derive(Debug)]
pub struct Array {
	pub(crate) header: ArrayHeader,
	pub(crate) data: [u8],
}

impl Array {
	pub fn length(&self) -> usize {
		self.header.length
	}
}

impl<'a> TryInto<&'a mut [i32]> for &mut Array {
	type Error = String;

	fn try_into(self) -> Result<&'a mut [i32], Self::Error> {
		let clr = self.header.element_type.clr();
		let internal = clr.internal();
		let p = internal.primitives();

		if self.header.element_type == p.int {
			Ok(unsafe {
				slice::from_raw_parts_mut(self.data.as_ptr() as *mut i32, self.header.length)
			})
		} else {
			Err(format!(
				"Cannot convert {}[] into int[]",
				self.header.element_type
			))
		}
	}
}
