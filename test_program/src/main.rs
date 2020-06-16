use clr_lite::pe::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/MethodDefTests/bin/Debug/netcoreapp3.1/MethodDefTests.dll"
	);

	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let pe = PeInfo::parse(data).unwrap();
	let cli_header = pe.cli_header();
	let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

	let type_defs = &metadata.tables.type_def;
	let fields = &metadata.tables.field;
	let methods = &metadata.tables.method_def;

	// Print all defined types and their fields
	for (i, td) in type_defs.rows().iter().enumerate() {
		let name = metadata.strings_heap.get(td.type_name).unwrap();
		let namespace = metadata.strings_heap.get(td.type_namespace).unwrap();
		let extends = match td.extends {
			clr_lite::metadata::TypeDefOrRef::TypeRefHandle(t) => metadata
				.strings_heap
				.get(metadata.tables.type_ref[t].type_name),
			clr_lite::metadata::TypeDefOrRef::TypeDefHandle(t) => type_defs
				.get(t)
				.map(|t| t.type_name)
				.and_then(|n| metadata.strings_heap.get(n)),
			_ => Some(""),
		};
		println!(
			"{} {}{} : {:?}",
			i + 1,
			if namespace.is_empty() {
				String::new()
			} else {
				format!("{}.", namespace)
			},
			name,
			extends
		);

		let field_end = if i == type_defs.rows().len() - 1 {
			fields.rows().len() + 1
		} else {
			type_defs.rows()[i + 1].field_list.0
		};

		println!("{} fields:", field_end - td.field_list.0);
		for i in td.field_list.0..field_end {
			let field = &fields[i.into()];
			let name = metadata.strings_heap.get(field.name).unwrap();
			let sig = metadata.blob_heap.get_field_sig(field.signature);
			println!("\t{}, {:?}", name, sig);
		}

		let method_end = if i == type_defs.rows().len() - 1 {
			methods.rows().len() + 1
		} else {
			type_defs.rows()[i + 1].method_list.0
		};

		println!("{} methods:", method_end - td.method_list.0);
		for i in td.method_list.0..method_end {
			let method = &methods[i.into()];
			let name = metadata.strings_heap.get(method.name).unwrap();
			println!(
				"\t{} {:?}",
				name,
				metadata.blob_heap.get_method_def_sig(method.signature)
			);
		}

		if i != type_defs.rows().len() - 1 {
			println!();
		}
	}

	dbg!(metadata.tables.param);
}
