use crate::vm::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod stack;
use stack::*;

pub mod value;
pub use value::*;

pub(crate) struct Interpreter {
	pub(crate) clr: Option<Weak<RefCell<ClrInternal>>>,
	stack: Vec<u8>,
}

impl Interpreter {
	pub(crate) fn new() -> Interpreter {
		Interpreter {
			clr: None,
			stack: Vec::with_capacity(1024 * 1024),
		}
	}

	pub fn execute(&mut self, m: Method) -> Result<(), String> {
		StackFrame::new(self.clr(), m, &mut self.stack).execute()
	}

	fn clr(&self) -> ClrLite {
		ClrLite(self.clr.as_ref().unwrap().upgrade().unwrap())
	}
}
