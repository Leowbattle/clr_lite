use clr_lite::pe::*;

fn main() {
	// let data = include_bytes!(
	// 	"../../tests/metadata/tables/FieldTests/bin/Debug/netcoreapp3.1/FieldTests.dll"
	// );

	let data = include_bytes!(
		"../../tests/metadata/blob/FieldSigTests/bin/Debug/netcoreapp3.1/FieldSigTests.dll"
	);

	let pe = PeInfo::parse(data).unwrap();
	let cli_header = pe.cli_header();
	let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

	let type_defs = metadata.tables.type_def.as_ref().unwrap();
	let fields = metadata.tables.field.as_ref().unwrap();

	// Print all defined types and their fields
	for (i, td) in type_defs.rows().iter().enumerate() {
		let name = metadata.strings_heap.get(td.type_name).unwrap();
		let namespace = metadata.strings_heap.get(td.type_namespace).unwrap();
		let extends = match td.extends {
			clr_lite::metadata::TypeDefOrRef::TypeRefHandle(t) => metadata
				.strings_heap
				.get(metadata.tables.type_ref.as_ref().unwrap()[t].type_name),
			clr_lite::metadata::TypeDefOrRef::TypeDefHandle(t) => type_defs
				.get(t)
				.map(|t| t.type_name)
				.and_then(|n| metadata.strings_heap.get(n)),
			_ => None,
		};
		println!(": {}.{} : {:?}", namespace, name, extends);

		let field_end = if i == type_defs.rows().len() - 1 {
			fields.rows().len() + 1
		} else {
			type_defs.rows()[i + 1].field_list.0
		};

		for i in td.field_list.0..field_end {
			let field = &fields[i.into()];
			let name = metadata.strings_heap.get(field.name).unwrap();
			let sig = metadata.blob_heap.get_field_sig(field.signature);
			println!("\t: {}, {:?}", name, sig);
		}

		println!("");
	}
}
