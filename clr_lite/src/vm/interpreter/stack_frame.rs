use crate::vm::gc::*;
use crate::vm::interpreter::*;
use crate::vm::reflection::*;
use crate::vm::*;

use std::mem::size_of;
use std::slice;

use num_traits::FromPrimitive;

pub(super) struct StackFrame<'a> {
	clr: ClrLite,
	interpreter: &'a mut Interpreter,
	method: Method,
	amount_allocated: usize,

	code: Rc<[u8]>,
	ip: usize,
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

	pub fn execute(&mut self) -> RunResult {
		let body = match self.method.implementation() {
			MethodImplementation::IL(b) => b,
			_ => unreachable!(),
		};

		// Allocate storage for local variables on the managed stack.
		let mut locals = unsafe {
			let count = body.local_variables().len();
			slice::from_raw_parts_mut(
				self.stackalloc(count * size_of::<Value>()).as_mut_ptr() as *mut Value,
				count,
			)
		};

		// let mut params = unsafe {
		// 	let count = self.method.parameters().len();
		// 	let start = self.interpreter.operand_stack.len() - count;
		// 	slice::from_raw_parts_mut(
		// 		self.interpreter
		// 			.operand_stack
		// 			.as_mut_ptr()
		// 			.offset(start as isize) as *mut Value,
		// 		count,
		// 	)
		// };

		loop {
			let op = self.get_opcode();
			match op {
				Opcodes::Nop => {}

				// TODO Box struct return values
				Opcodes::Ret => {
					return if self.method.return_type() == self.clr.get_type("System.Void") {
						Ok(None)
					} else {
						Ok(self.try_pop())
					}
				}

				Opcodes::Ldc_I4_M1 => self.push(Value::I32(-1)),
				Opcodes::Ldc_I4_0 => self.push(Value::I32(0)),
				Opcodes::Ldc_I4_1 => self.push(Value::I32(1)),
				Opcodes::Ldc_I4_2 => self.push(Value::I32(2)),
				Opcodes::Ldc_I4_3 => self.push(Value::I32(3)),
				Opcodes::Ldc_I4_4 => self.push(Value::I32(4)),
				Opcodes::Ldc_I4_5 => self.push(Value::I32(5)),
				Opcodes::Ldc_I4_6 => self.push(Value::I32(6)),
				Opcodes::Ldc_I4_7 => self.push(Value::I32(7)),
				Opcodes::Ldc_I4_8 => self.push(Value::I32(8)),
				Opcodes::Ldc_I4_S => {
					let val = self.il_get::<i8>() as i32;
					self.push(Value::I32(val));
				}
				Opcodes::Ldc_I4 => {
					let val = self.il_get::<i32>();
					self.push(Value::I32(val));
				}
				Opcodes::Ldc_I8 => {
					let val = self.il_get::<i64>();
					self.push(Value::I64(val));
				}
				Opcodes::Ldc_R4 => {
					let val = self.il_get::<f32>();
					self.push(Value::F32(val));
				}
				Opcodes::Ldc_R8 => {
					let val = self.il_get::<f64>();
					self.push(Value::F64(val));
				}

				Opcodes::Ldloc => {
					let i = self.il_get::<u16>() as usize;
					self.push(locals[i]);
				}
				Opcodes::Ldloc_S => {
					let i = self.il_get::<u8>() as usize;
					self.push(locals[i]);
				}
				Opcodes::Ldloc_0 => self.push(locals[0]),
				Opcodes::Ldloc_1 => self.push(locals[1]),
				Opcodes::Ldloc_2 => self.push(locals[2]),
				Opcodes::Ldloc_3 => self.push(locals[3]),
				Opcodes::Stloc => {
					let i = self.il_get::<u16>() as usize;
					locals[i] = self.pop();
				}
				Opcodes::Stloc_S => {
					let i = self.il_get::<u8>() as usize;
					locals[i] = self.pop();
				}
				Opcodes::Stloc_0 => locals[0] = self.pop(),
				Opcodes::Stloc_1 => locals[1] = self.pop(),
				Opcodes::Stloc_2 => locals[2] = self.pop(),
				Opcodes::Stloc_3 => locals[3] = self.pop(),

				Opcodes::Br_S => {
					let offset = self.il_get::<i8>();
					self.ip = ((self.ip as isize) + offset as isize) as usize;
				}

				_ => return Err(format!("Use of unimplemented instruction {:?}", op)),
			}
		}
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

	/// Gets a value of type T from the IL
	fn il_get<T: Copy>(&mut self) -> T {
		let val = unsafe { *(self.code.as_ptr().offset(self.ip as isize) as *mut T) };
		self.ip += size_of::<T>();
		val
	}

	fn push(&mut self, v: Value) {
		self.interpreter.operand_stack.push(v);
	}

	fn pop(&mut self) -> Value {
		self.try_pop().unwrap()
	}

	fn try_pop(&mut self) -> Option<Value> {
		self.interpreter.operand_stack.pop()
	}

	fn stackalloc<'b>(&'b mut self, size: usize) -> &'b mut [u8] {
		let base = self.interpreter.stackalloc.len();
		self.interpreter.stackalloc.resize(base + size, 0);
		let data = &mut self.interpreter.stackalloc[base..base + size];
		self.amount_allocated += size;
		data
	}
}

impl<'a> Drop for StackFrame<'a> {
	fn drop(&mut self) {
		// Free all stack memory used
		self.interpreter
			.stackalloc
			.truncate(self.interpreter.stackalloc.len() - self.amount_allocated)
	}
}
