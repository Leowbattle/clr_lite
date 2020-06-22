use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/blob/FieldSignatureTests/bin/Debug/netcoreapp3.1/FieldSignatureTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");
	// let data =
	// 	include_bytes!(r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\3.1.2\mscorlib.dll");

	let metadata = Metadata::read(data).unwrap();

	for f in metadata.tables().field.rows() {
		println!(
			"{}, {:?}",
			metadata.strings().get(f.name).unwrap(),
			metadata
				.blob()
				.new_reader(f.signature)
				.unwrap()
				.read_field_signature()
				.unwrap()
		);
	}
}
