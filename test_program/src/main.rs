use clr_lite::metadata::tables::*;
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

	for m in metadata.tables().method_semantics.rows() {
		println!(
			"{:-<50} is {:?} for {}",
			metadata
				.strings()
				.get(metadata.tables().method_def[m.method].name)
				.unwrap(),
			m.semantics,
			match m.association {
				HasSemanticsHandle::EventHandle(e) => metadata
					.strings()
					.get(metadata.tables().event[e].name)
					.unwrap(),
				HasSemanticsHandle::PropertyHandle(p) => metadata
					.strings()
					.get(metadata.tables().property[p].name)
					.unwrap(),
			}
		);
	}

	println!("{:?}", d);
}
