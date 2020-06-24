use clr_lite::vm::*;

fn main() {
	let mut rt = ClrLite::new_runtime().unwrap();
	for a in rt.assemblies() {
		println!("{}", a.name());
	}
}
