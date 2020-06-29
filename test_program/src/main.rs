use clr_lite::vm::*;

fn main() {
	let mut rt = ClrLite::new_runtime().unwrap();
	let a = rt
		.load_assembly_from_path("tests/EmptyExe/bin/Debug/netcoreapp3.1/EmptyExe.dll")
		.unwrap();
	rt.execute(a.entry_point().unwrap()).unwrap();

	// for a in rt.assemblies().iter() {
	// 	println!("{}", a.name());
	// 	for t in a.types().iter() {
	// 		if let Some(base) = t.base() {
	// 			print!("\t{} {} : {}", t.kind(), t, base);
	// 		} else {
	// 			print!("\t{} {}", t.kind(), t);
	// 		}
	// 		for i in t.interfaces().iter() {
	// 			print!(", {}", i);
	// 		}
	// 		println!();
	// 		println!("\t\tsize = {}", t.size());
	// 		for f in t.fields().iter() {
	// 			println!("\t\t{}", f);
	// 		}
	// 		for m in t.methods().iter() {
	// 			println!("\t\t{}", m);
	// 		}
	// 	}

	// 	if let Some(ep) = a.entry_point() {
	// 		println!(
	// 			"\tEntry point = {}.{}",
	// 			ep.declaring_type().unwrap().full_name(),
	// 			ep.name()
	// 		);
	// 	}
	// }

	// println!("number of loaded types = {}", rt.types().len());
}
