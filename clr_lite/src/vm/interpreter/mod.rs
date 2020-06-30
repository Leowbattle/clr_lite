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
	pub stackalloc: Vec<u8>,
	pub operand_stack: Vec<Value>,
}

impl Interpreter {
	pub(crate) fn new() -> Interpreter {
		Interpreter {
			clr: None,
			stackalloc: Vec::with_capacity(1024 * 1024),
			operand_stack: Vec::with_capacity(128 * 1024),
		}
	}

	pub fn execute(&mut self, m: Method, params: &mut [Value]) -> RunResult {
		// Validate parameters
		if m.parameters().len() != params.len() {
			return Err(format!(
				"Invalid parameter count {} for {}",
				params.len(),
				m,
			));
		}
		for (_p1, _p2) in m.parameters().iter().zip(params.iter()) {
			// TODO Check
		}

		StackFrame::new(self.clr(), self, m).execute(params)
	}

	fn clr(&self) -> ClrLite {
		ClrLite(self.clr.as_ref().unwrap().upgrade().unwrap())
	}
}

#[cfg(test)]
mod tests;
