use crate::metadata::{tables::TableType, MetadataToken};
use crate::vm::interpreter::*;

use std::mem::size_of;
use std::slice;

use num_traits::FromPrimitive;

pub(super) struct StackFrame<'a> {
	clr: ClrLite,
	interpreter: &'a mut Interpreter,
	gc: Rc<RefCell<GcHeap>>,
	assembly: Assembly,
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

		let gc = interpreter.gc.clone();

		StackFrame {
			clr,
			interpreter,
			gc,
			assembly: method.declaring_type().unwrap().assembly().unwrap(),
			method,
			amount_allocated: 0,
			code,
			ip: 0,
		}
	}

	pub fn execute(&mut self, params: &mut [Value]) -> RunResult {
		let body = match self.method.implementation() {
			MethodImplementation::IL(b) => b,
			_ => unreachable!(),
		};

		// Allocate storage for local variables on the managed stack.
		let locals = unsafe {
			let count = body.local_variables().len();
			slice::from_raw_parts_mut(
				self.stackalloc(count * size_of::<Value>()).as_mut_ptr() as *mut Value,
				count,
			)
		};

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

				Opcodes::Ldarg_S => {
					let i = self.il_get::<u8>() as usize;
					self.push(params[i]);
				}
				Opcodes::Ldarg => {
					let i = self.il_get::<u32>() as usize;
					self.push(params[i]);
				}
				Opcodes::Ldarg_0 => self.push(params[0]),
				Opcodes::Ldarg_1 => self.push(params[1]),
				Opcodes::Ldarg_2 => self.push(params[2]),
				Opcodes::Ldarg_3 => self.push(params[3]),
				Opcodes::Starg_S => {
					let i = self.il_get::<u8>() as usize;
					params[i] = self.pop();
				}
				Opcodes::Starg => {
					let i = self.il_get::<u32>() as usize;
					params[i] = self.pop();
				}

				Opcodes::Dup => self.push(self.peek()),
				Opcodes::Pop => {
					self.pop();
				}

				// TODO Limit recursion to stop stack overflow
				Opcodes::Call => {
					let token = self.il_get::<MetadataToken>();

					self.call_method(token)?;
				}

				Opcodes::Br_S => {
					let offset = self.il_get::<i8>();
					self.ip = ((self.ip as isize) + offset as isize) as usize;
				}
				Opcodes::Br => {
					let offset = self.il_get::<i32>();
					self.ip = ((self.ip as isize) + offset as isize) as usize;
				}
				Opcodes::Brfalse => {
					let offset = self.il_get::<i32>();
					if self.pop().is_null_or_zero() {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Brfalse_S => {
					let offset = self.il_get::<i8>();
					if self.pop().is_null_or_zero() {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Brtrue => {
					let offset = self.il_get::<i32>();
					if !self.pop().is_null_or_zero() {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Brtrue_S => {
					let offset = self.il_get::<i8>();
					if !self.pop().is_null_or_zero() {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Beq => {
					let a = self.pop();
					let b = self.pop();
					let offset = self.il_get::<i32>();
					if a == b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Beq_S => {
					let a = self.pop();
					let b = self.pop();
					let offset = self.il_get::<i8>();
					if a == b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Bge => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i32>();
					if a >= b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Bge_S => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i8>();
					if a >= b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Bgt => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i32>();
					if a > b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Bgt_S => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i8>();
					if a > b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Ble => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i32>();
					if a <= b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Ble_S => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i8>();
					if a <= b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Blt => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i32>();
					if a < b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}
				Opcodes::Blt_S => {
					let b = self.pop();
					let a = self.pop();
					let offset = self.il_get::<i8>();
					if a < b {
						self.ip = ((self.ip as isize) + offset as isize) as usize;
					}
				}

				Opcodes::Ceq => {
					let a = self.pop();
					let b = self.pop();
					self.push(Value::I32((a == b) as i32));
				}
				Opcodes::Cgt | Opcodes::Cgt_Un => {
					let b = self.pop();
					let a = self.pop();
					self.push(Value::I32((a > b) as i32));
				}
				Opcodes::Clt | Opcodes::Clt_Un => {
					let b = self.pop();
					let a = self.pop();
					self.push(Value::I32((a < b) as i32));
				}

				Opcodes::Add => {
					let b = self.pop();
					let a = self.pop();
					self.push(a + b);
				}
				Opcodes::Sub => {
					let b = self.pop();
					let a = self.pop();
					self.push(a - b);
				}
				Opcodes::Mul => {
					let b = self.pop();
					let a = self.pop();
					self.push(a * b);
				}
				Opcodes::Div | Opcodes::Div_Un => {
					let b = self.pop();
					let a = self.pop();
					self.push(a / b);
				}
				Opcodes::Rem | Opcodes::Rem_Un => {
					let b = self.pop();
					let a = self.pop();
					self.push(a % b);
				}
				Opcodes::Neg => {
					let val = self.pop();
					self.push(-val);
				}

				Opcodes::And => {
					let b = self.pop();
					let a = self.pop();
					self.push(a & b);
				}
				Opcodes::Or => {
					let b = self.pop();
					let a = self.pop();
					self.push(a | b);
				}
				Opcodes::Xor => {
					let b = self.pop();
					let a = self.pop();
					self.push(a ^ b);
				}
				Opcodes::Not => {
					let val = self.pop();
					self.push(!val);
				}
				Opcodes::Shl => {
					let b = self.pop();
					let a = self.pop();
					self.push(a << b);
				}
				Opcodes::Shr => {
					let b = self.pop();
					let a = self.pop();
					self.push(a >> b);
				}

				Opcodes::Newobj => {
					let ctor_token = self.il_get::<MetadataToken>();
					let ctor = self.assembly.resolve_method(ctor_token).unwrap();
					let t = ctor.declaring_type().unwrap();
					let mut o = &mut *self.gc.borrow_mut().alloc(t) as *mut Object;

					// In instance methods, this is arg0, so insert the object reference before the other arguments.
					self.interpreter.operand_stack.insert(
						self.interpreter.operand_stack.len() - ctor.parameters().len(),
						Value::Object(o),
					);
					self.call_method(ctor_token)?;
					self.push(Value::Object(o));
				}

				_ => {
					return Err(format!(
						"Use of unimplemented instruction {:?} at IL_{:04x}",
						op,
						self.ip - 1
					))
				}
			}
		}
	}

	fn call_method(&mut self, token: MetadataToken) -> Result<(), String> {
		let method = self
			.assembly
			.resolve_method(token)
			.ok_or_else(|| format!("Unable to find method for token {}", token))?;

		// Allocate space on the stack for parameters
		let params = unsafe {
			let param_count = if method.is_static() {
				method.parameters().len()
			} else {
				method.parameters().len() + 1
			};
			let data = self.stackalloc(param_count * size_of::<Value>());
			slice::from_raw_parts_mut(data.as_ptr() as *mut Value, param_count)
		};

		// Pop parameters off the operand stack and into the parameters
		for p in params.iter_mut().rev() {
			*p = self.pop();
		}

		let ret = self.interpreter.execute(method, params)?;
		if let Some(value) = ret {
			self.push(value);
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

	fn peek(&self) -> Value {
		self.try_peek().unwrap()
	}

	fn try_peek(&self) -> Option<Value> {
		Some(*self.interpreter.operand_stack.last()?)
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
