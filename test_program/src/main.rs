use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/ClassLayoutTests/bin/Debug/netcoreapp3.1/ClassLayoutTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	dbg!(metadata.tables().standalone_sig.rows().len());

	for f in metadata.tables().field_layout.rows() {
		println!(
			"{} offset = {}",
			metadata
				.strings()
				.get(metadata.tables().field[f.field].name)
				.unwrap(),
			f.offset
		);
	}
}
