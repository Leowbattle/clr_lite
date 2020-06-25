use clr_lite::vm::*;

fn main() {
	let mut rt = ClrLite::new_runtime().unwrap();
	rt.load_assembly_from_path("tests/EmptyExe/bin/Debug/netcoreapp3.1/EmptyExe.dll")
		.unwrap();
	for a in rt.assemblies() {
		println!("{}", a.name());
		for t in a.types() {
			if let Some(base) = t.base() {
				println!("\t{} {} : {}", t.kind(), t, base);
			} else {
				println!("\t{} {}", t.kind(), t);
			}
			for f in t.fields() {
				println!("\t\t{}", f);
			}
		}
	}

	println!("number of loaded types = {}", rt.types().count());
}
