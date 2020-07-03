#![allow(non_upper_case_globals)]
use crate::vm::gc::*;

use std::mem;

bitflags! {
	pub struct ObjectFlags: u8 {
		/// Used in garbage collection.
		const GC_Marked = 0x1;

		/// If any GC handles exist, the object is pinned.
		const GC_Handles_Exist = 0x2;

		const IsArray = 0x4;
	}
}

/// Annoyingly I cannot use #[repr(packed)], which would reduce the size due to less padding.
/// However unaligned pointers are undefined behaviour and caused an access violation.
#[derive(Debug)]
pub(crate) struct ObjectHeader {
	pub flags: ObjectFlags,
	pub object_type: Type,
}

#[derive(Debug)]
pub struct Object {
	pub(crate) header: ObjectHeader,
	pub(crate) raw: RawObject,
}

impl Object {
	pub fn as_array<'a>(&'a mut self) -> Option<&'a mut Array> {
		if self.header.flags.contains(ObjectFlags::IsArray) {
			Some(unsafe { mem::transmute(self) })
		} else {
			None
		}
	}
}
