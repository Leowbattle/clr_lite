use clr_lite::metadata::Metadata;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/MethodDefTests/bin/Debug/netcoreapp3.1/MethodDefTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for m in metadata.tables().method_def.rows() {
		println!("{} {:?}", metadata.strings().get(m.name).unwrap(), m);
	}
}
