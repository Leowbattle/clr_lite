use super::*;

use std::collections::HashMap;

// Named this way because there is already write! in Rust.
fn console_write(clr: &mut ClrLite, params: &mut [Value]) -> Result<Option<Value>, String> {
	let s = match params[0] {
		Value::Object(o) => String::from_utf16(o.as_string().unwrap().data()).unwrap(),
		_ => unimplemented!(),
	};
	print!("{}", s);
	clr.internal_mut().output.push_str(&s);
	Ok(None)
}

pub(super) fn load_internal_methods(h: &mut HashMap<String, InternalMethod>) {
	h.insert(
		"Void System.Console.Write(System.String)".to_string(),
		console_write,
	);
}
