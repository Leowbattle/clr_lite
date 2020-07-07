#![allow(non_snake_case)]
use super::*;

use std::collections::HashMap;

fn get_Item(clr: &mut ClrLite, params: &mut [Value]) -> Result<Option<Value>, String> {
	unimplemented!()
}

fn get_Length(clr: &mut ClrLite, params: &mut [Value]) -> Result<Option<Value>, String> {
	unimplemented!()
}

pub(super) fn load_internal_methods(h: &mut HashMap<String, InternalMethod>) {
	h.insert(
		"Char System.String.get_Item(System.Int32)".to_string(),
		get_Item,
	);
	h.insert("Int32 System.String.get_Length()".to_string(), get_Length);
}
