use super::*;

use std::slice;

bitflags! {
	pub struct ObjectFlags : u8 {
		const NONE = 0x0;

		const GC_MARKED = 0x1;

		const IS_ARRAY = 0x2;
		const IS_STRING = 0x4;
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct ObjectHeader {
	pub flags: ObjectFlags,
	pub type_id: TypeID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Object(pub(crate) *mut ObjectHeader);

impl Object {
	fn as_raw_object(&mut self) -> Option<RawObject> {
		unsafe {
			if self.header().flags & (ObjectFlags::IS_ARRAY | ObjectFlags::IS_STRING)
				== ObjectFlags::NONE
			{
				Some(RawObject(self.0.offset(1) as *mut u8))
			} else {
				None
			}
		}
	}

	pub(crate) fn raw_data<'a>(&'a self, clr: &ClrLite) -> &'a [u8] {
		let size = if let Some(arr) = self.as_array() {
			mem::size_of::<ArrayHeader>()
				+ (clr.types()[arr.header().element_type_id as usize].size() * arr.header().length)
		} else if let Some(s) = self.as_string() {
			mem::size_of::<StringHeader>() + (mem::size_of::<u16>() * s.data().len())
		} else {
			mem::size_of::<ObjectHeader>() + self.type_of(clr).size()
		};
		unsafe { slice::from_raw_parts(self.0 as *const u8, size) }
	}

	pub fn type_of(&self, clr: &ClrLite) -> Type {
		clr.types()[self.header().type_id as usize].clone()
	}

	pub fn header<'a>(&'a self) -> &'a ObjectHeader {
		unsafe { &*self.0 }
	}

	pub fn header_mut<'a>(&'a mut self) -> &'a mut ObjectHeader {
		unsafe { &mut *self.0 }
	}

	pub fn as_array(&self) -> Option<Array> {
		if self.header().flags.contains(ObjectFlags::IS_ARRAY) {
			Some(Array(self.0 as *mut ArrayHeader))
		} else {
			None
		}
	}

	pub fn as_string(&self) -> Option<ManagedString> {
		if self.header().flags.contains(ObjectFlags::IS_STRING) {
			Some(ManagedString(self.0 as *mut StringHeader))
		} else {
			None
		}
	}

	pub fn set(&mut self, field: Field, value: Value) {
		self.as_raw_object().unwrap().set(field, value);
	}

	pub fn get(&mut self, field: Field, clr: &ClrLite) -> Value {
		self.as_raw_object().unwrap().get(field, clr)
	}
}
