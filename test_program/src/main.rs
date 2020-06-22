use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/blob/MethodDefSignatureTests/bin/Debug/netcoreapp3.1/MethodDefSignatureTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");
	// let data =
	// 	include_bytes!(r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\3.1.2\mscorlib.dll");

	let metadata = Metadata::read(data).unwrap();

	for m in metadata.tables().method_def.rows() {
		println!(
			"{}, {:?}",
			metadata.strings().get(m.name).unwrap(),
			metadata
				.blob()
				.new_reader(m.signature)
				.unwrap()
				.read_method_def_signature()
				.unwrap()
		);
	}
}
