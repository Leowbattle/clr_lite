use super::*;
use crate::vm::reflection::HasManagedType;

use std::error;
use std::fmt;
use std::slice;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct ArrayHeader {
	pub header: ObjectHeader,
	pub element_type_id: TypeID,
	pub length: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Array(pub(crate) *mut ArrayHeader);

#[derive(Debug)]
pub enum ArrayError {
	IndexOutOfRange,
	TypeMismatch,
}

impl fmt::Display for ArrayError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl error::Error for ArrayError {}

impl Array {
	pub fn as_object(&self) -> Object {
		Object(self.0 as *mut ObjectHeader)
	}

	pub fn type_of(&self, clr: &ClrLite) -> Type {
		clr.types()[self.header().header.type_id as usize].clone()
	}

	pub fn header<'a>(&'a self) -> &'a ArrayHeader {
		unsafe { &*self.0 }
	}

	pub fn header_mut<'a>(&'a mut self) -> &'a mut ArrayHeader {
		unsafe { &mut *self.0 }
	}

	pub fn set(&mut self, i: usize, value: Value, clr: &ClrLite) -> Result<(), ArrayError> {
		let header = unsafe { &mut *self.0 };
		if i >= header.length {
			return Err(ArrayError::IndexOutOfRange);
		}

		let internal = clr.internal();
		let _p = internal.primitives();

		unsafe {
			match value {
				Value::I8(x) => {
					*(self
						.as_mut_slice::<i8>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::U8(x) => {
					*(self
						.as_mut_slice::<u8>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::I16(x) => {
					*(self
						.as_mut_slice::<i16>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::U16(x) => {
					*(self
						.as_mut_slice::<u16>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::I32(x) => {
					*(self
						.as_mut_slice::<i32>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::U32(x) => {
					*(self
						.as_mut_slice::<u32>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::I64(x) => {
					*(self
						.as_mut_slice::<i64>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::U64(x) => {
					*(self
						.as_mut_slice::<u64>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::F32(x) => {
					*(self
						.as_mut_slice::<f32>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::F64(x) => {
					*(self
						.as_mut_slice::<f64>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = x
				}
				Value::Object(o) => {
					*(self
						.as_mut_slice::<Object>(clr)?
						.as_mut_ptr()
						.offset(i as isize)) = o
				}
			};
		}

		Ok(())
	}

	pub fn as_slice<'a, T: HeldInArray<'a>>(
		&'a self,
		clr: &ClrLite,
	) -> Result<&'a [T], ArrayError> {
		T::as_slice(self, clr)
	}

	pub fn as_mut_slice<'a, T: HeldInArrayMut<'a>>(
		&'a self,
		clr: &ClrLite,
	) -> Result<&'a mut [T], ArrayError> {
		T::as_mut_slice(self, clr)
	}
}

pub trait HeldInArray<'a>: Sized {
	fn as_slice(arr: &Array, clr: &ClrLite) -> Result<&'a [Self], ArrayError>;
}

pub trait HeldInArrayMut<'a>: HeldInArray<'a> {
	fn as_mut_slice(arr: &Array, clr: &ClrLite) -> Result<&'a mut [Self], ArrayError>;
}

macro_rules! impl_array_to_slice {
	($($t:ty),*) => {
		$(impl<'a> HeldInArray<'a> for $t {
			fn as_slice(arr: &Array, clr: &ClrLite) -> Result<&'a [$t], ArrayError> {
				let mt = <$t>::managed_type(clr);
				if arr.header().element_type_id == mt.id() {
					unsafe {
						Ok(slice::from_raw_parts(arr.0.offset(1) as *const _, arr.header().length))
					}
				}
				else {
					Err(ArrayError::TypeMismatch)
				}
			}
		}

		impl<'a> HeldInArrayMut<'a> for $t {
			fn as_mut_slice(arr: &Array, clr: &ClrLite) -> Result<&'a mut [$t], ArrayError> {
				let mt = <$t>::managed_type(clr);
				if arr.header().element_type_id == mt.id() {
					unsafe {
						Ok(slice::from_raw_parts_mut(arr.0.offset(1) as *mut _, arr.header().length))
					}
				}
				else {
					Err(ArrayError::TypeMismatch)
				}
			}
		})*
	};
}

impl_array_to_slice! {
	i8,
	i16,
	i32,
	i64,
	u8,
	u16,
	u32,
	u64,
	f32,
	f64,
	Object
}
