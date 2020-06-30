use crate::vm::interpreter::*;

fn run(func_name: &str) -> Result<Option<Value>, String> {
	let mut clr = ClrLite::new_runtime().unwrap();
	let _a = clr
		.load_assembly_from_path(
			"../tests/vm/InterpreterTests/bin/Debug/netcoreapp3.1/InterpreterTests.dll",
		)
		.unwrap();
	let tests = clr.get_type("InterpreterTests.Tests").unwrap();
	clr.execute(tests.get_method(func_name).unwrap())
}

#[test]
fn test_empty() {
	assert_eq!(run("Empty"), Ok(None));
}

#[test]
fn test_ldc_i4_m1() {
	assert_eq!(run("Ldc_I4_M1"), Ok(Some(Value::I32(-1))));
}

#[test]
fn test_ldc_i4_4() {
	assert_eq!(run("Ldc_I4_4"), Ok(Some(Value::I32(4))));
}

#[test]
fn test_ldc_i4_s() {
	assert_eq!(run("Ldc_I4_S"), Ok(Some(Value::I32(100))));
}

#[test]
fn test_ldc_i4() {
	assert_eq!(run("Ldc_I4"), Ok(Some(Value::I32(1000))));
}

#[test]
fn test_ldc_r4() {
	assert_eq!(run("Ldc_R4"), Ok(Some(Value::F32(3.14159))));
}

#[test]
fn test_ldc_r8() {
	assert_eq!(run("Ldc_R8"), Ok(Some(Value::F64(1.2345678))));
}

#[test]
fn test_locals() {
	assert_eq!(run("Locals"), Ok(Some(Value::I32(42))));
}
