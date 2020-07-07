#![allow(non_snake_case)]
///! Any C# method marked with [MethodImpl(MethodImplOptions.InternalCall)] is implemented in native code in this module.
mod System_Console;
mod System_String;

use crate::vm::*;

pub type InternalMethod = fn(&mut ClrLite, &mut [Value]) -> Result<Option<Value>, String>;

use std::collections::HashMap;

pub(crate) fn load_internal_methods() -> HashMap<String, InternalMethod> {
	let mut h = HashMap::new();
	System_String::load_internal_methods(&mut h);
	System_Console::load_internal_methods(&mut h);
	h
}
