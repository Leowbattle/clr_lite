use super::*;

use std::slice;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct StringHeader {
	pub header: ObjectHeader,
	pub length: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ManagedString(pub(crate) *mut StringHeader);

impl ManagedString {
	pub fn as_object(&self) -> Object {
		Object(self.0 as *mut ObjectHeader)
	}

	pub fn header<'a>(&'a self) -> &'a StringHeader {
		unsafe { &*self.0 }
	}

	pub fn header_mut<'a>(&'a mut self) -> &'a mut StringHeader {
		unsafe { &mut *self.0 }
	}

	pub fn data<'a>(&'a self) -> &'a [u16] {
		unsafe { slice::from_raw_parts(self.0.offset(1) as *const _, self.header().length) }
	}

	/// This function is unsafe because C# strings are immutable.
	/// This should only be used to set the string data when it is being created.
	pub(crate) unsafe fn data_mut<'a>(&'a mut self) -> &'a mut [u16] {
		slice::from_raw_parts_mut(self.0.offset(1) as *mut _, self.header().length)
	}
}
