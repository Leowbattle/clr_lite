use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/FieldMarshalTests/bin/Debug/netcoreapp3.1/FieldMarshalTests.dll"
	);
	let data =
		include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	dbg!(metadata.tables().decl_security.rows().len());

	for f in metadata.tables().field_marshal.rows() {
		println!(
			"{}",
			metadata
				.strings()
				.get(match f.parent {
					HasFieldMarshalHandle::FieldHandle(f) => {
						metadata.tables().field[f].name
					}
					HasFieldMarshalHandle::ParamHandle(p) => {
						metadata.tables().param[p].name
					}
				})
				.unwrap()
		);
	}
}
