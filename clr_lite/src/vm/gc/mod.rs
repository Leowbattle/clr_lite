///! Garbage collector
use crate::vm::*;

use std::cell::RefCell;
use std::rc::Weak;

pub mod raw_object;
pub use raw_object::*;

pub(crate) struct GcHeap {
	pub(crate) clr: Option<Weak<RefCell<ClrInternal>>>,
}

impl GcHeap {
	pub(crate) fn new() -> GcHeap {
		GcHeap { clr: None }
	}

	fn clr(&self) -> Option<ClrLite> {
		Some(ClrLite(self.clr.as_ref()?.upgrade()?))
	}
}
