use crate::vm::gc::*;
use crate::vm::reflection::*;
use crate::vm::*;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

mod stack_frame;
use stack_frame::*;

pub mod value;
pub use value::*;

pub mod opcodes;
pub use opcodes::*;

pub(crate) struct Interpreter {
	pub clr: ClrLite,
	pub gc: Rc<RefCell<GcHeap>>,
	pub stackalloc: Vec<u8>,
	pub operand_stack: Vec<Value>,
}

// TODO Initialise constants
// TODO Call static constructors

impl Interpreter {
	pub(crate) fn new(clr: ClrLite) -> Interpreter {
		Interpreter {
			clr: clr.clone(),
			gc: Rc::new(RefCell::new(GcHeap::new(clr, 1024))),
			stackalloc: Vec::with_capacity(1024),
			operand_stack: Vec::with_capacity(8),
		}
	}

	pub fn execute(&mut self, m: Method, params: &mut [Value]) -> Result<Option<Value>, String> {
		match m.implementation() {
			MethodImplementation::IL(_) => StackFrame::new(self.clr(), self, m).execute(params),
			MethodImplementation::Internal(fn_ptr) => fn_ptr(&mut self.clr, params),
			_ => unimplemented!("{}", m),
		}
	}

	fn clr(&self) -> ClrLite {
		self.clr.clone()
	}

	fn gc<'a>(&'a mut self) -> RefMut<'a, GcHeap> {
		self.gc.borrow_mut()
	}
}

#[cfg(test)]
mod tests;
