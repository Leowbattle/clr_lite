use crate::vm::interpreter::*;

pub enum Value<'a> {
	I32(i32),
	ValueType(RawObject<'a>),
}
