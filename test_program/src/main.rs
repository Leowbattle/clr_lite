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

	for g in metadata.tables().generic_param.rows() {
		println!("{}, {:?}", metadata.strings().get(g.name).unwrap(), g);
	}

	println!("{:?}", d);
}
