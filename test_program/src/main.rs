use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/PropertyTests/bin/Debug/netcoreapp3.1/PropertyTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let start = std::time::Instant::now();
	let metadata = Metadata::read(data).unwrap();
	let end = std::time::Instant::now();
	let d = end - start;
	println!("{:?}", d);

	for (i, p) in metadata.tables().property_map.rows().iter().enumerate() {
		println!(
			"{:?}.{}",
			metadata
				.strings()
				.get(metadata.tables().type_def[p.parent].namespace)
				.unwrap(),
			metadata
				.strings()
				.get(metadata.tables().type_def[p.parent].name)
				.unwrap()
		);

		let property_end = if i == metadata.tables().property_map.rows().len() - 1 {
			metadata.tables().property.rows().len()
		} else {
			usize::from(metadata.tables().property_map.rows()[i + 1].property_list) - 1
		};

		for i in usize::from(p.property_list) - 1..property_end {
			println!(
				"\t{}",
				metadata
					.strings()
					.get(metadata.tables().property[i.into()].name)
					.unwrap()
			);
		}
	}
}
