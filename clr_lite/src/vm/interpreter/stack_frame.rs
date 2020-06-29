use crate::vm::gc::*;
use crate::vm::interpreter::*;
use crate::vm::reflection::*;
use crate::vm::*;

use std::mem::size_of;

use num_traits::FromPrimitive;

pub(super) struct StackFrame<'a> {
	clr: ClrLite,
	interpreter: &'a mut Interpreter,
	method: Method,
	amount_allocated: usize,

	code: Rc<[u8]>,
	ip: usize,
	// params: &'a [Value<'a>],
	// locals: &'a [Value<'a>]
}

impl<'a> StackFrame<'a> {
	pub fn new(clr: ClrLite, interpreter: &'a mut Interpreter, method: Method) -> StackFrame<'a> {
		let code = match method.implementation() {
			MethodImplementation::IL(b) => b.code(),
			_ => unimplemented!(),
		};

		StackFrame {
			clr,
			interpreter,
			method,
			amount_allocated: 0,
			code,
			ip: 0,
		}
	}

	pub fn execute(&mut self) -> Result<(), String> {
		// let data = [0; 8];
		// let t = self.clr.get_type("EmptyExe.Position").unwrap();
		// let x = t.get_field("x").unwrap();
		// let y = t.get_field("y").unwrap();
		// let mut r = RawObject::new(t, &data);
		// r.set(x, Value::I32(3));
		// r.set(y, Value::I32(42));

		loop {
			let op = self.get_opcode();
			match op {
				Opcodes::Nop => {}
				Opcodes::Ret => return Ok(()),
				_ => return Err(format!("Use of unimplemented instruction {:?}", op)),
			}
		}

		Ok(())
	}

	fn get_opcode(&mut self) -> Opcodes {
		let val = self.code[self.ip] as u16;
		self.ip += 1;
		if val > Opcodes::Prefix7 as u16 {
			let op = Opcodes::from_u16((self.code[self.ip] as u16) | (val << 8)).unwrap();
			self.ip += 1;
			op
		} else {
			Opcodes::from_u16(val).unwrap()
		}
	}

	fn stackalloc(&'a mut self, size: usize) -> &'a [u8] {
		self.interpreter.stack.reserve(size);
		let data = &self.interpreter.stack
			[self.interpreter.stack.len()..self.interpreter.stack.len() + size];
		self.amount_allocated += size;
		data
	}
}

impl<'a> Drop for StackFrame<'a> {
	fn drop(&mut self) {
		// Free all stack memory used
		self.interpreter
			.stack
			.truncate(self.interpreter.stack.len() - self.amount_allocated)
	}
}
