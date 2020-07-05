use super::*;

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
	flags: ObjectFlags,
	type_id: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Object(pub(crate) *mut ObjectHeader);

impl Object {
	fn as_raw_object(&mut self) -> Option<RawObject> {
		unsafe {
			let h = *self.0;
			if h.flags & (ObjectFlags::IS_ARRAY | ObjectFlags::IS_STRING) == ObjectFlags::NONE {
				Some(RawObject(self.0.offset(1) as *mut u8))
			} else {
				None
			}
		}
	}

	pub fn set(&mut self, field: Field, value: Value) {
		self.as_raw_object().unwrap().set(field, value);
	}

	pub fn get(&mut self, field: Field, clr: &ClrLite) -> Value {
		self.as_raw_object().unwrap().get(field, clr)
	}
}
