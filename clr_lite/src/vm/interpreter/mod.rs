use crate::vm::gc::*;
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
	pub clr: ClrLite,
	pub gc: GcHeap,
	pub stackalloc: Vec<u8>,
	pub operand_stack: Vec<Value>,
}

impl Interpreter {
	pub(crate) fn new(clr: ClrLite) -> Interpreter {
		Interpreter {
			clr,
			gc: GcHeap::new(1024 * 1024),
			stackalloc: Vec::with_capacity(1024 * 1024),
			operand_stack: Vec::with_capacity(128 * 1024),
		}
	}

	pub fn execute(&mut self, m: Method, params: &mut [Value]) -> RunResult {
		StackFrame::new(self.clr(), self, m).execute(params)
	}

	fn clr(&self) -> ClrLite {
		self.clr.clone()
	}

	fn gc<'a>(&'a mut self) -> &'a mut GcHeap {
		&mut self.gc
	}
}

#[cfg(test)]
mod tests;
