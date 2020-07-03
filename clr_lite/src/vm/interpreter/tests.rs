use crate::vm::interpreter::*;

const DATA: &'static [u8] = include_bytes!(
	"../../../../tests/vm/InterpreterTests/bin/Debug/netcoreapp3.1/InterpreterTests.dll"
);

fn run(func_name: &str, params: &mut [Value]) -> Result<Option<Value>, String> {
	let mut clr = ClrLite::new_runtime().unwrap();
	let _a = clr.load_assembly_from_data(DATA).unwrap();
	let tests = clr.get_type("InterpreterTests.Tests").unwrap();
	clr.execute(tests.get_method(func_name).unwrap(), params)
}

#[test]
fn test_empty() {
	assert_eq!(run("Empty", &mut []), Ok(None));
}

#[test]
fn test_non_static() {
	assert!(matches!(run("NonStatic", &mut []), Err(_)));
}

#[test]
fn test_fibonacci() {
	assert_eq!(
		run("Fibonacci", &mut [Value::I32(10)]),
		Ok(Some(Value::I32(55)))
	);
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

#[test]
fn test_add() {
	assert_eq!(
		run("Add", &mut [Value::I32(4), Value::I32(8)]),
		Ok(Some(Value::I32(12)))
	);
	assert_ne!(
		run("Add", &mut [Value::I32(9), Value::I32(10)]),
		Ok(Some(Value::I32(21)))
	);

	// I don't compare directly to 3.3f32 because you shouldn't check floats for equality with some constant using ==.
	assert_eq!(
		run("AddFloat", &mut [Value::F32(1.1f32), Value::F32(2.2f32)]),
		Ok(Some(Value::F32(1.1f32 + 2.2f32)))
	);
}

#[test]
fn test_sub() {
	assert_eq!(
		run("Sub", &mut [Value::I32(4), Value::I32(8)]),
		Ok(Some(Value::I32(-4)))
	);
}

#[test]
fn test_mul() {
	assert_eq!(
		run("Mul", &mut [Value::I32(4), Value::I32(8)]),
		Ok(Some(Value::I32(32)))
	);
}

#[test]
fn test_div() {
	assert_eq!(
		run("Div", &mut [Value::I32(100), Value::I32(4)]),
		Ok(Some(Value::I32(25)))
	);
	assert_eq!(
		run("Div_Un", &mut [Value::I32(100), Value::I32(4)]),
		Ok(Some(Value::I32(25)))
	);
}

#[test]
fn test_rem() {
	assert_eq!(run("IsEven", &mut [Value::I32(4)]), Ok(Some(Value::I32(1))));
	assert_eq!(run("IsEven", &mut [Value::I32(5)]), Ok(Some(Value::I32(0))));

	assert_eq!(
		run("IsEven_Un", &mut [Value::I32(100)]),
		Ok(Some(Value::I32(1)))
	);
	assert_eq!(
		run("IsEven_Un", &mut [Value::I32(55)]),
		Ok(Some(Value::I32(0)))
	);
}

#[test]
fn test_negate() {
	assert_eq!(
		run("Negate", &mut [Value::I32(1)]),
		Ok(Some(Value::I32(-1)))
	);
	assert_eq!(
		run("Negate", &mut [Value::I32(-55)]),
		Ok(Some(Value::I32(55)))
	);

	assert_eq!(
		run("NegateFloat", &mut [Value::F32(3.14159)]),
		Ok(Some(Value::F32(-3.14159)))
	);
}

#[test]
fn test_bitwise() {
	assert_eq!(
		run("BitAnd", &mut [Value::I32(10), Value::I32(14)]),
		Ok(Some(Value::I32(10 & 14)))
	);

	assert_eq!(
		run("BitOr", &mut [Value::I32(10), Value::I32(14)]),
		Ok(Some(Value::I32(10 | 14)))
	);

	assert_eq!(
		run("BitXor", &mut [Value::I32(10), Value::I32(14)]),
		Ok(Some(Value::I32(10 ^ 14)))
	);

	assert_eq!(
		run("BitNot", &mut [Value::I32(10)]),
		Ok(Some(Value::I32(!10)))
	);

	assert_eq!(
		run("Shl", &mut [Value::I32(1000), Value::I32(2)]),
		Ok(Some(Value::I32(1000 << 2)))
	);

	assert_eq!(
		run("Shr", &mut [Value::I32(1000), Value::I32(2)]),
		Ok(Some(Value::I32(1000 >> 2)))
	);
}

#[test]
fn test_logic_and() {
	assert_eq!(
		run("LogicAnd", &mut [Value::I32(0), Value::I32(0)]),
		Ok(Some(Value::I32(0)))
	);
	assert_eq!(
		run("LogicAnd", &mut [Value::I32(0), Value::I32(1)]),
		Ok(Some(Value::I32(0)))
	);
	assert_eq!(
		run("LogicAnd", &mut [Value::I32(1), Value::I32(0)]),
		Ok(Some(Value::I32(0)))
	);
	assert_eq!(
		run("LogicAnd", &mut [Value::I32(1), Value::I32(1)]),
		Ok(Some(Value::I32(1)))
	);
}

#[test]
fn test_logic_or() {
	assert_eq!(
		run("LogicOr", &mut [Value::I32(0), Value::I32(0)]),
		Ok(Some(Value::I32(0)))
	);
	assert_eq!(
		run("LogicOr", &mut [Value::I32(0), Value::I32(1)]),
		Ok(Some(Value::I32(1)))
	);
	assert_eq!(
		run("LogicOr", &mut [Value::I32(1), Value::I32(0)]),
		Ok(Some(Value::I32(1)))
	);
	assert_eq!(
		run("LogicOr", &mut [Value::I32(1), Value::I32(1)]),
		Ok(Some(Value::I32(1)))
	);
}

#[test]
fn test_logic_not() {
	assert_eq!(
		run("LogicNot", &mut [Value::I32(0)]),
		Ok(Some(Value::I32(1)))
	);
	assert_eq!(
		run("LogicNot", &mut [Value::I32(1)]),
		Ok(Some(Value::I32(0)))
	);
}

#[test]
fn test_create_object() {
	assert_eq!(run("CreateObject", &mut []), Ok(None));
}

#[test]
fn test_create_custom_object() {
	assert_eq!(
		run("CreateCustomObject", &mut [Value::I32(2)]),
		Ok(Some(Value::I32(3)))
	);
}

#[test]
fn test_create_array() {
	assert_eq!(
		run("CreateArray", &mut [Value::I32(10)]),
		Ok(Some(Value::I32(10)))
	);
}

#[test]
fn test_array_indexing() {
	assert_eq!(
		run("ArrayIndexing", &mut [Value::I32(10), Value::I32(3)]),
		Ok(Some(Value::I32(3)))
	);
}
