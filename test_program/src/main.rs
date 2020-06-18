use clr_lite::metadata::Metadata;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/ParamTests/bin/Debug/netcoreapp3.1/ParamTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for p in metadata.tables().param.rows() {
		println!("{}", metadata.strings().get(p.name).unwrap());
	}
}
