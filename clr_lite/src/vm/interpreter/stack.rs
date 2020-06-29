use crate::vm::gc::*;
use crate::vm::reflection::*;
use crate::vm::*;

use std::mem::size_of;

pub(super) struct StackFrame<'a> {
	clr: ClrLite,
	method: Method,
	stack: &'a mut Vec<u8>,
	amount_allocated: usize,
}

impl<'a> StackFrame<'a> {
	pub fn new(clr: ClrLite, method: Method, stack: &'a mut Vec<u8>) -> StackFrame<'a> {
		StackFrame {
			clr,
			method,
			stack,
			amount_allocated: 0,
		}
	}

	pub fn execute(&mut self) -> Result<(), String> {
		let data = [0; 8];
		let t = self.clr.get_type("EmptyExe.Position").unwrap();
		let x = t.get_field("x").unwrap();
		let y = t.get_field("y").unwrap();
		let mut r = RawObject::new(t, &data);
		r.set(x, Value::I32(3));
		r.set(y, Value::I32(42));

		Ok(())
	}

	fn stackalloc(&'a mut self, size: usize) -> &'a [u8] {
		self.stack.reserve(size);
		let data = &self.stack[self.stack.len()..self.stack.len() + size];
		self.amount_allocated += size;
		data
	}
}

impl<'a> Drop for StackFrame<'a> {
	fn drop(&mut self) {
		// Free all stack memory used
		self.stack
			.truncate(self.stack.len() - self.amount_allocated)
	}
}
