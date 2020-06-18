use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/MemberRefTests/bin/Debug/netcoreapp3.1/MemberRefTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for m in metadata.tables().member_ref.rows() {
		println!(
			"{}.{}",
			match m.class {
				MemberRefParentHandle::TypeRefHandle(t) => metadata
					.strings()
					.get(metadata.tables().type_ref[t].name)
					.unwrap(),
				_ => "unimplemented",
			},
			metadata.strings().get(m.name).unwrap()
		);
	}
}
