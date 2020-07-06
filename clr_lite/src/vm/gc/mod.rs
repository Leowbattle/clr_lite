///! Garbage collector
use crate::vm::*;

use std::cell::RefCell;
use std::rc::Weak;

pub mod raw_object;
pub use raw_object::*;

pub mod object;
pub use object::*;

pub mod array;
pub use array::*;

use std::mem;

pub(crate) struct GcHeap {
	pub(crate) clr: Option<Weak<RefCell<ClrInternal>>>,
	memory: Vec<u8>,
}

impl GcHeap {
	pub(crate) fn new(size: usize) -> GcHeap {
		GcHeap {
			clr: None,
			memory: Vec::with_capacity(size),
		}
	}

	fn ensure_size(&mut self, size: usize) {
		if self.memory.len() + size > self.memory.capacity() {
			self.collect();
		}
		if self.memory.len() + size > self.memory.capacity() {
			self.memory.reserve(self.memory.capacity());
		}
	}

	pub fn alloc(&mut self, t: Type) -> Object {
		let size = mem::size_of::<ObjectHeader>() + t.size();
		self.ensure_size(size);
		let base = self.memory.len();

		// 0-initialise memory for object
		self.memory.resize(base + size, 0);

		unsafe {
			let mut obj =
				Object(self.memory.as_mut_ptr().offset(base as isize) as *mut ObjectHeader);
			let header = obj.header_mut();
			header.type_id = t.id();
			obj
		}
	}

	pub fn alloc_array(&mut self, element_type: Type, length: usize) -> Array {
		let size = mem::size_of::<ArrayHeader>() + (element_type.size() * length);
		self.ensure_size(size);
		let base = self.memory.len();

		// 0-initialise memory for array
		self.memory.resize(base + size, 0);

		unsafe {
			let mut arr = Array(self.memory.as_mut_ptr().offset(base as isize) as *mut ArrayHeader);
			let header = arr.header_mut();
			header.header.flags |= ObjectFlags::IS_ARRAY;
			header.header.type_id = element_type.array_of().id();
			header.element_type_id = element_type.id();
			header.length = length;
			arr
		}
	}

	pub fn collect(&mut self) {}

	fn clr(&self) -> Option<ClrLite> {
		Some(ClrLite(self.clr.as_ref()?.upgrade()?))
	}
}
