use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/MethodImplTests/bin/Debug/netcoreapp3.1/MethodImplTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let start = std::time::Instant::now();
	let metadata = Metadata::read(data).unwrap();
	let end = std::time::Instant::now();
	let d = end - start;

	for m in metadata.tables().method_impl.rows() {
		println!(
			"{}: {} overrides {}",
			metadata
				.strings()
				.get(metadata.tables().type_def[m.class].name)
				.unwrap(),
			metadata
				.strings()
				.get(match m.body {
					MethodDefOrRefHandle::MethodDefHandle(m) =>
						metadata.tables().method_def[m].name,
					MethodDefOrRefHandle::MemberRefHandle(m) =>
						metadata.tables().member_ref[m].name,
				})
				.unwrap(),
			metadata
				.strings()
				.get(match m.declaration {
					MethodDefOrRefHandle::MethodDefHandle(m) =>
						metadata.tables().method_def[m].name,
					MethodDefOrRefHandle::MemberRefHandle(m) =>
						metadata.tables().member_ref[m].name,
				})
				.unwrap()
		);
	}

	println!("{:?}", d);
}
