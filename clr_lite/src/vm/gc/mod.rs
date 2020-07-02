///! Garbage collector
use crate::vm::*;

use std::cell::RefCell;
use std::rc::Weak;

pub mod raw_object;
pub use raw_object::*;

pub mod object;
pub use object::*;

use std::mem;
use std::ops::DerefMut;

pub struct GcHeap {
	pub(crate) clr: Option<Weak<RefCell<ClrInternal>>>,
	memory: Vec<u8>,
}

pub struct GcHandle<'heap> {
	heap: &'heap GcHeap,
	object: &'heap mut Object,
}

impl Deref for GcHandle<'_> {
	type Target = Object;

	fn deref(&self) -> &Object {
		self.object
	}
}

impl DerefMut for GcHandle<'_> {
	fn deref_mut(&mut self) -> &mut Object {
		self.object
	}
}

impl GcHeap {
	pub(crate) fn new(size: usize) -> GcHeap {
		GcHeap {
			clr: None,
			memory: Vec::with_capacity(size),
		}
	}

	pub fn alloc<'a>(&'a mut self, object_type: Type) -> GcHandle<'a> {
		let size = object_type.size() + mem::size_of::<ObjectHeader>();

		// If there is not enough space to allocate this object, collect garbage.
		if self.memory.len() + size > self.memory.capacity() {
			dbg!("Run GC");
			self.collect();
		}
		// If there is still not enough memory, allocate more.
		if self.memory.len() + size > self.memory.capacity() {
			dbg!("Allocate more heap memory");
			self.memory.reserve(self.memory.capacity());
		}

		let object = unsafe {
			let offset = self.memory.len();
			// 0 initialise memory for object.
			self.memory.resize(self.memory.len() + size, 0);
			let data = &mut self.memory[offset..offset + size];
			mem::transmute(data)
		};

		GcHandle { heap: self, object }
	}

	pub fn collect(&mut self) {}

	fn clr(&self) -> ClrLite {
		ClrLite(self.clr.as_ref().unwrap().upgrade().unwrap())
	}
}
