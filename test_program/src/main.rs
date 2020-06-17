use clr_lite::metadata::Metadata;

fn main() {
	let data = include_bytes!("../../tests/EmptyExe/bin/Debug/netcoreapp3.1/EmptyExe.dll");
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	dbg!(metadata
		.strings()
		.get(metadata.tables().module[0.into()].name));
}
