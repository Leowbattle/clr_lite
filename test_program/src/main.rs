use clr_lite::metadata::Metadata;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/FieldTests/bin/Debug/netcoreapp3.1/FieldTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for f in metadata.tables().field.rows() {
		println!(
			"{:<15} {:?}",
			metadata.strings().get(f.name).unwrap(),
			f.attributes,
		);
	}
}
