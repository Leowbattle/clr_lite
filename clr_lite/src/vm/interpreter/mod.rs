use crate::vm::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod stack_frame;
use stack_frame::*;

pub mod value;
pub use value::*;

pub mod opcodes;
pub use opcodes::*;

pub type RunResult = Result<Option<Value>, String>;

pub(crate) struct Interpreter {
	pub clr: Option<Weak<RefCell<ClrInternal>>>,
	pub gc: Rc<RefCell<GcHeap>>,
	pub stackalloc: Vec<u8>,
	pub operand_stack: Vec<Value>,
}

impl Interpreter {
	pub(crate) fn new(gc: Rc<RefCell<GcHeap>>) -> Interpreter {
		Interpreter {
			clr: None,
			gc,
			stackalloc: Vec::with_capacity(1024 * 1024),
			operand_stack: Vec::with_capacity(128 * 1024),
		}
	}

	pub fn execute(&mut self, m: Method, params: &mut [Value]) -> RunResult {
		StackFrame::new(self.clr(), self, m).execute(params)
	}

	fn clr(&self) -> ClrLite {
		ClrLite(self.clr.as_ref().unwrap().upgrade().unwrap())
	}
}

#[cfg(test)]
mod tests;
