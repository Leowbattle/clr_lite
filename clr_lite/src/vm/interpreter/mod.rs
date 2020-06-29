use crate::vm::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod stack_frame;
use stack_frame::*;

pub mod value;
pub use value::*;

pub mod opcodes;
pub use opcodes::*;

pub(crate) struct Interpreter {
	pub clr: Option<Weak<RefCell<ClrInternal>>>,
	pub stack: Vec<u8>,
}

impl Interpreter {
	pub(crate) fn new() -> Interpreter {
		Interpreter {
			clr: None,
			stack: Vec::with_capacity(1024 * 1024),
		}
	}

	pub fn execute(&mut self, m: Method) -> Result<(), String> {
		StackFrame::new(self.clr(), self, m).execute()
	}

	fn clr(&self) -> ClrLite {
		ClrLite(self.clr.as_ref().unwrap().upgrade().unwrap())
	}
}
