use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/ConstantTests/bin/Debug/netcoreapp3.1/ConstantTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();
	for c in metadata.tables().constant.rows() {
		let parent = match c.parent {
			HasConstantHandle::FieldHandle(f) => metadata
				.strings()
				.get(metadata.tables().field[f].name)
				.unwrap(),
			_ => "unimplemented",
		};
		let mut br = metadata.blob().new_reader(c.value).unwrap();
		let value = match c.r#type {
			ConstantType::Int => br.read::<u32>().unwrap().to_string(),
			ConstantType::String => String::from_utf16(br.read_utf16_str().unwrap()).unwrap(),
			_ => "unimplemented".to_string(),
		};
		println!("{:-<50}{}", parent, value);
	}
}
