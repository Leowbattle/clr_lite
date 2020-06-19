use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/ClassLayoutTests/bin/Debug/netcoreapp3.1/ClassLayoutTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();

	for l in metadata.tables().class_layout.rows() {
		println!(
			"{} pack = {:?} size = {:?}",
			metadata
				.strings()
				.get(metadata.tables().type_def[l.parent].name)
				.unwrap(),
			l.packing_size,
			l.class_size
		);
	}
}
