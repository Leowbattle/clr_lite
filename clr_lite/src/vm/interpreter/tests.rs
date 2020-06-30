use crate::vm::interpreter::*;

fn run(func_name: &str, params: &mut [Value]) -> Result<Option<Value>, String> {
	let mut clr = ClrLite::new_runtime().unwrap();
	let _a = clr
		.load_assembly_from_path(
			"../tests/vm/InterpreterTests/bin/Debug/netcoreapp3.1/InterpreterTests.dll",
		)
		.unwrap();
	let tests = clr.get_type("InterpreterTests.Tests").unwrap();
	clr.execute(tests.get_method(func_name).unwrap(), params)
}

#[test]
fn test_empty() {
	assert_eq!(run("Empty", &mut []), Ok(None));
}

#[test]
fn test_ldc_i4_m1() {
	assert_eq!(run("Ldc_I4_M1", &mut []), Ok(Some(Value::I32(-1))));
}

#[test]
fn test_ldc_i4_4() {
	assert_eq!(run("Ldc_I4_4", &mut []), Ok(Some(Value::I32(4))));
}

#[test]
fn test_ldc_i4_s() {
	assert_eq!(run("Ldc_I4_S", &mut []), Ok(Some(Value::I32(100))));
}

#[test]
fn test_ldc_i4() {
	assert_eq!(run("Ldc_I4", &mut []), Ok(Some(Value::I32(1000))));
}

#[test]
fn test_ldc_r4() {
	assert_eq!(run("Ldc_R4", &mut []), Ok(Some(Value::F32(3.14159))));
}

#[test]
fn test_ldc_r8() {
	assert_eq!(run("Ldc_R8", &mut []), Ok(Some(Value::F64(1.2345678))));
}

#[test]
fn test_locals() {
	assert_eq!(run("Locals", &mut []), Ok(Some(Value::I32(42))));
}

#[test]
fn test_parameters() {
	assert_eq!(
		run("Parameters", &mut [Value::I32(0); 256]),
		Ok(Some(Value::I32(10)))
	);
}

#[test]
fn test_call_empty() {
	assert_eq!(run("CallEmpty", &mut []), Ok(None));
}

#[test]
fn test_return_value() {
	assert_eq!(run("ReturnValue", &mut []), Ok(Some(Value::F32(3.14159))));
}

#[test]
fn test_goto() {
	assert_eq!(run("Goto", &mut []), Ok(Some(Value::I32(4))));
}

#[test]
fn test_br() {
	assert_eq!(run("Br", &mut []), Ok(Some(Value::I32(4))));
}

#[test]
fn test_br_false_s() {
	assert_eq!(
		run("Brfalse_S", &mut [Value::I32(0)]),
		Ok(Some(Value::I32(1)))
	);
	assert_eq!(
		run("Brfalse_S", &mut [Value::I32(1)]),
		Ok(Some(Value::I32(0)))
	);
}

#[test]
fn test_br_true_s() {
	assert_eq!(
		run("Brtrue_S", &mut [Value::I32(0)]),
		Ok(Some(Value::I32(0)))
	);
	assert_eq!(
		run("Brtrue_S", &mut [Value::I32(1)]),
		Ok(Some(Value::I32(1)))
	);
}

#[test]
fn test_gt() {
	assert_eq!(run("Gt", &mut [Value::I32(4)]), Ok(Some(Value::I32(4))));
	assert_eq!(run("Gt", &mut [Value::I32(100)]), Ok(Some(Value::I32(5))));
}

#[test]
fn test_lt() {
	assert_eq!(run("Lt", &mut [Value::I32(4)]), Ok(Some(Value::I32(5))));
	assert_eq!(run("Lt", &mut [Value::I32(100)]), Ok(Some(Value::I32(4))));
}

#[test]
fn test_ge() {
	assert_eq!(run("Ge", &mut [Value::I32(4)]), Ok(Some(Value::I32(4))));
	assert_eq!(run("Ge", &mut [Value::I32(100)]), Ok(Some(Value::I32(5))));
}

#[test]
fn test_le() {
	assert_eq!(run("Le", &mut [Value::I32(4)]), Ok(Some(Value::I32(5))));
	assert_eq!(run("Le", &mut [Value::I32(100)]), Ok(Some(Value::I32(4))));
}
