use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/ManifestResourceTests/bin/Debug/netcoreapp3.1/ManifestResourceTests.dll"
	);
	let data =
		include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");
	// let data =
	// 	include_bytes!(r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\3.1.2\mscorlib.dll");

	let start = std::time::Instant::now();
	let metadata = Metadata::read(data).unwrap();
	let end = std::time::Instant::now();
	let d = end - start;

	for nc in metadata.tables().nested_class.rows() {
		println!(
			"{}.{}",
			metadata
				.strings()
				.get(metadata.tables().type_def[nc.enclosing].name)
				.unwrap(),
			metadata
				.strings()
				.get(metadata.tables().type_def[nc.nested].name)
				.unwrap()
		);
	}

	println!("{:?}", d);
}
