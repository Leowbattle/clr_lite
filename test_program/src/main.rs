use clr_lite::metadata::Metadata;

fn main() {
	let data = include_bytes!("../../tests/EmptyExe/bin/Debug/netcoreapp3.1/EmptyExe.dll");
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for t in metadata.tables().type_ref.rows() {
		println!("{}", metadata.strings().get(t.name).unwrap());
	}
}
