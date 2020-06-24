use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const LIBS: &'static [&'static str] = &["System.Runtime"];

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let profile = env::var("PROFILE").unwrap();

	for lib in LIBS {
		let path = match profile.as_str() {
			"debug" => format!("../std/{0}/bin/Debug/netcoreapp3.1/{0}.dll", lib),
			"release" => format!("../std/{0}/bin/Release/netcoreapp3.1/{0}.dll", lib),
			_ => unreachable!(),
		};

		println!("cargo:rerun-if-changed={}", path);
		fs::copy(
			path,
			PathBuf::from_str(&out_dir)
				.unwrap()
				.join(format!("{}.dll", lib)),
		)
		.unwrap();
	}
}
