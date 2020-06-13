use clr_lite::pe::*;

fn main() {
	// let data =
	// 	include_bytes!(r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\3.1.2\System.dll");
	let data =
		include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");

	let pe = PeInfo::parse(data).unwrap();
	let cli_header = pe.cli_header();
	let metadata = cli_header.and_then(|c| c.metadata());
	dbg!(metadata);
}
