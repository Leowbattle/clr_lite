use super::*;

use std::collections::HashMap;

// Named this way because there is already write! in Rust.
// TODO Make this output into a String buffer or something to capture output, which could also be redirected to stdout.
fn console_write(clr: &mut ClrLite, params: &mut [Value]) -> Result<Option<Value>, String> {
	match params[0] {
		Value::Object(o) => print!(
			"{}",
			String::from_utf16(o.as_string().unwrap().data()).unwrap()
		),
		_ => unimplemented!(),
	}
	Ok(None)
}

pub(super) fn load_internal_methods(h: &mut HashMap<String, InternalMethod>) {
	h.insert(
		"Void System.Console.Write(System.String)".to_string(),
		console_write,
	);
}
