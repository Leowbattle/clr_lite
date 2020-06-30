use clr_lite::vm::*;

fn main() {
	let mut clr = ClrLite::new_runtime().unwrap();
	let a = clr
		.load_assembly_from_path(std::env::args().nth(1).unwrap())
		.unwrap();
	dbg!(clr.execute(a.entry_point().unwrap()));
}
