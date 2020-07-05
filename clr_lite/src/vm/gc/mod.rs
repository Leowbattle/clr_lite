///! Garbage collector
use crate::vm::*;

use std::cell::RefCell;
use std::rc::Weak;

pub mod raw_object;
pub use raw_object::*;

pub mod object;
pub use object::*;

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
		let base = self.memory.len();

		// 0-initialise memory for object
		self.memory.resize(base + size, 0);

		unsafe { Object(self.memory.as_mut_ptr().offset(base as isize) as *mut ObjectHeader) }
	}

	pub fn collect(&mut self) {}

	fn clr(&self) -> Option<ClrLite> {
		Some(ClrLite(self.clr.as_ref()?.upgrade()?))
	}
}
