use clr_lite::pe::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/TypeRefTests/bin/Debug/netcoreapp3.1/TypeRefTests.dll"
	);

	let pe = PeInfo::parse(data).unwrap();
	let cli_header = pe.cli_header();
	let metadata = cli_header.and_then(|c| c.metadata()).unwrap();
	//dbg!(&metadata);

	// let strings = metadata.strings_heap;
	// let types = metadata.tables.type_def.unwrap();

	// let types = types
	// 	.rows()
	// 	.iter()
	// 	.map(|row| (strings.get(row.type_name).unwrap(), row))
	// 	.collect::<std::collections::HashMap<&str, &clr_lite::metadata::tables::TypeDef>>();

	// dbg!(types);

	for td in metadata.tables.type_def.as_ref().unwrap().rows().iter() {
		let name = metadata.strings_heap.get(td.type_name).unwrap();
		let namespace = metadata.strings_heap.get(td.type_namespace).unwrap();
		let extends = match td.extends {
			clr_lite::metadata::TypeDefOrRef::TypeRefHandle(t) => metadata
				.strings_heap
				.get(metadata.tables.type_ref.as_ref().unwrap()[t].type_name),
			clr_lite::metadata::TypeDefOrRef::TypeDefHandle(t) => metadata
				.tables
				.type_def
				.as_ref()
				.unwrap()
				.get(t)
				.map(|t| t.type_name)
				.and_then(|n| metadata.strings_heap.get(n)),
			_ => None,
		};
		println!("{}.{} : {:?}", namespace, name, extends);
	}
}
