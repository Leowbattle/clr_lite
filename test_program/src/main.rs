use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/CustomAttributeTests/bin/Debug/netcoreapp3.1/CustomAttributeTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();

	// Get a list of types and their methods
	let type_defs = metadata
		.tables()
		.type_def
		.rows()
		.iter()
		.enumerate()
		.map(|(i, t)| {
			(metadata.strings().get(t.name).unwrap(), {
				let method_start = t.method_list.into();
				let method_end = if method_start == metadata.tables().method_def.rows().len() {
					metadata.tables().method_def.rows().len()
				} else {
					metadata.tables().type_def[(i + 1).into()]
						.method_list
						.into()
				};
				(method_start..method_end)
					.map(|i| (i - 1).into())
					.collect::<Box<[MethodDefHandle]>>()
			})
		})
		.collect::<Box<[(&str, Box<[MethodDefHandle]>)]>>();

	for c in metadata.tables().custom_attribute.rows() {
		let parent = match c.parent {
			HasCustomAttributeHandle::FieldHandle(f) => metadata
				.strings()
				.get(metadata.tables().field[f].name)
				.unwrap(),
			HasCustomAttributeHandle::TypeRefHandle(t) => metadata
				.strings()
				.get(metadata.tables().type_ref[t].name)
				.unwrap(),
			HasCustomAttributeHandle::TypeDefHandle(t) => metadata
				.strings()
				.get(metadata.tables().type_def[t].name)
				.unwrap(),
			HasCustomAttributeHandle::AssemblyHandle(_) => "assembly",
			_ => "unimplemented",
		};
		let attribute_type = match c.attribute_type {
			CustomAttributeTypeHandle::MethodDefHandle(m) => {
				type_defs
					.iter()
					.flat_map(|t| t.1.iter().map(move |m2| (t.0, m2)))
					.find(|(_, &m2)| m == m2)
					.unwrap()
					.0
			}
			CustomAttributeTypeHandle::MemberRefHandle(m) => {
				match metadata.tables().member_ref[m].class {
					MemberRefParentHandle::TypeRefHandle(t) => metadata
						.strings()
						.get(metadata.tables().type_ref[t].name)
						.unwrap(),
					_ => "unimplemented",
				}
			}
		};

		println!("[{}] {}", attribute_type, parent);
	}
}
