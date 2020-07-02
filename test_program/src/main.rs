use clr_lite::vm::*;

const DATA: &'static [u8] =
	include_bytes!("../../tests/vm/InterpreterTests/bin/Debug/netcoreapp3.1/InterpreterTests.dll");

fn main() {
	let mut clr = ClrLite::new_runtime().unwrap();
	let a = clr.load_assembly_from_data(DATA).unwrap();
	dbg!(clr.execute(a.entry_point().unwrap(), &mut []));
}
