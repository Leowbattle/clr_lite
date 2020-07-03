use crate::vm::reflection::*;
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
use std::ops::DerefMut;
use std::slice;

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

	fn ensure_size(&mut self, size: usize) {
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
	}

	pub fn alloc<'a>(&'a mut self, object_type: Type) -> GcHandle<'a> {
		let size = object_type.size() + mem::size_of::<ObjectHeader>();
		self.ensure_size(size);

		let header = ObjectHeader {
			flags: ObjectFlags::empty(),
			object_type,
		};

		let object: &mut Object = unsafe {
			let offset = self.memory.len();
			// 0 initialise memory for object.
			self.memory.resize(self.memory.len() + size, 0);
			let data = &mut self.memory[offset..offset + size];

			// Initially I was setting the fields of the header after the mem::transmute, however
			// when I tried to set object.header.object_type I got a null-pointer exception.
			// I was very confused until I realised what was causing this.
			// In Rust when you assign to a variable, the old value is dropped, so the line
			// `object.header.object_type = object_type` causes `object.header.object_type` to be
			// dropped. However because the memory is zeroed, when the `Rc` destructor dereferences
			// its pointer to see if it should free it, it dereferences null.
			// After this line it is safe to assign to `object.header.object_type`, but this will never
			// happen anyway because you can't change the type of an object.
			data[0..mem::size_of::<ObjectHeader>()].copy_from_slice(slice::from_raw_parts(
				&header as *const _ as *const u8,
				mem::size_of::<ObjectHeader>(),
			));
			mem::transmute(data)
		};

		GcHandle { heap: self, object }
	}

	pub fn alloc_array<'a>(&'a mut self, element_type: Type, length: usize) -> GcHandle<'a> {
		let size = (element_type.size() * length) + mem::size_of::<ArrayHeader>();
		self.ensure_size(size);

		let header = ArrayHeader {
			object_header: ObjectHeader {
				flags: ObjectFlags::IsArray,
				object_type: element_type.get_array_type(),
			},
			element_type,
			length,
		};

		let array: &mut Array = unsafe {
			let offset = self.memory.len();
			// 0 initialise memory for array.
			self.memory.resize(self.memory.len() + size, 0);
			let data = &mut self.memory[offset..offset + size];
			data[0..mem::size_of::<ArrayHeader>()].copy_from_slice(slice::from_raw_parts(
				&header as *const _ as *const u8,
				mem::size_of::<ArrayHeader>(),
			));
			mem::transmute(data)
		};

		GcHandle {
			heap: self,
			object: unsafe { mem::transmute(array) },
		}
	}

	pub fn collect(&mut self) {}

	fn clr(&self) -> ClrLite {
		ClrLite(self.clr.as_ref().unwrap().upgrade().unwrap())
	}
}
