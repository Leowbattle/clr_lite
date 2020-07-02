#![allow(non_upper_case_globals)]
use crate::vm::gc::*;
use crate::vm::interpreter::*;
use crate::vm::reflection::*;

bitflags! {
	struct ObjectFlags: u8 {
		/// Used in garbage collection.
		const GC_Marked = 0x1;

		/// If any GC handles exist, the object is pinned.
		const GC_Handles_Exist = 0x2;
	}
}

#[repr(packed)]
pub(crate) struct ObjectHeader {
	flags: ObjectFlags,
	object_type: Type,
}

pub struct Object {
	pub(crate) header: ObjectHeader,
	pub(crate) raw: RawObject,
}
