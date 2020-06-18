use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/InterfaceImplTests/bin/Debug/netcoreapp3.1/InterfaceImplTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for i in metadata.tables().interface_impl.rows() {
		println!(
			"{} : {}",
			metadata
				.strings()
				.get(metadata.tables().type_def[i.class].name)
				.unwrap(),
			metadata
				.strings()
				.get(match i.interface {
					TypeDefOrRefHandle::TypeDefHandle(t) => metadata.tables().type_def[t].name,
					TypeDefOrRefHandle::TypeRefHandle(t) => metadata.tables().type_ref[t].name,
					_ => metadata.tables().module[0.into()].name,
				})
				.unwrap()
		);
	}
}
