use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/tables/EventTests/bin/Debug/netcoreapp3.1/EventTests.dll"
	);
	let data =
		include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let metadata = Metadata::read(data).unwrap();

	for (i, e) in metadata.tables().event_map.rows().iter().enumerate() {
		println!(
			"{:?}.{}",
			metadata
				.strings()
				.get(metadata.tables().type_def[e.parent].namespace)
				.unwrap(),
			metadata
				.strings()
				.get(metadata.tables().type_def[e.parent].name)
				.unwrap()
		);

		let event_end = if i == metadata.tables().event_map.rows().len() - 1 {
			metadata.tables().event.rows().len()
		} else {
			usize::from(metadata.tables().event_map.rows()[i + 1].event_list) - 1
		};

		for i in usize::from(e.event_list) - 1..event_end {
			println!(
				"\t{}",
				metadata
					.strings()
					.get(metadata.tables().event[i.into()].name)
					.unwrap()
			);
		}
	}
}
