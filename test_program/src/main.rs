use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/MethodImplTests/bin/Debug/netcoreapp3.1/MethodImplTests.dll"
	);
	let data =
		include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let start = std::time::Instant::now();
	let metadata = Metadata::read(data).unwrap();
	let end = std::time::Instant::now();
	let d = end - start;

	for i in metadata.tables().impl_map.rows() {
		println!(
			"{} in {}",
			metadata.strings().get(i.name).unwrap(),
			metadata
				.strings()
				.get(metadata.tables().module_ref[i.module].name)
				.unwrap()
		);
	}
	println!("{:?}", d);
}
