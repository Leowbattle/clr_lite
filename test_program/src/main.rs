use clr_lite::pe::*;

fn main() {
	let data =
		include_bytes!(r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\3.1.2\System.dll");

	let pe = PeInfo::parse(data);
	dbg!(pe);
}
