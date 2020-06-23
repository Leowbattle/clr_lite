use clr_lite::metadata::blob::ElementType;
use clr_lite::metadata::tables::*;
use clr_lite::metadata::*;

fn main() {
	let data = include_bytes!(
		"../../tests/metadata/blob/GenericInstanceTests/bin/Debug/netcoreapp3.1/GenericInstanceTests.dll"
	);
	// let data =
	// 	include_bytes!("C:/Program Files (x86)/steam/steamapps/common/Terraria/Terraria.exe");
	// let data =
	// 	include_bytes!(r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\3.1.2\mscorlib.dll");

	let metadata = Metadata::read(data).unwrap();

	for m in metadata.tables().method_spec.rows() {
		println!("{}", stringify_method_spec(&metadata, m));
	}
}

fn stringify_method_spec<'a>(metadata: &'a Metadata<'a>, m: &MethodSpec) -> String {
	use std::fmt::Write;

	let mut s = String::new();
	write!(
		&mut s,
		"{}<",
		metadata
			.strings()
			.get(match m.method {
				MethodDefOrRefHandle::MethodDefHandle(m) => metadata.tables().method_def[m].name,
				MethodDefOrRefHandle::MemberRefHandle(m) => metadata.tables().member_ref[m].name,
			})
			.unwrap()
	)
	.unwrap();

	let args = metadata
		.blob()
		.new_reader(m.instantiation)
		.unwrap()
		.read_method_spec()
		.unwrap()
		.args;
	for arg in args.iter() {
		write!(&mut s, "{}, ", stringify_element_type(metadata, arg)).unwrap();
	}
	s.truncate(s.len() - 2);
	write!(&mut s, ">").unwrap();
	s
}

fn stringify_element_type<'a>(metadata: &'a Metadata<'a>, e: &ElementType) -> String {
	use std::fmt::Write;
	match e {
		ElementType::ValueType(t) | ElementType::Class(t) => metadata
			.strings()
			.get(match t {
				TypeDefOrRefHandle::TypeDefHandle(t) => metadata.tables().type_def[*t].name,
				TypeDefOrRefHandle::TypeRefHandle(t) => metadata.tables().type_ref[*t].name,
				_ => unreachable!(),
			})
			.unwrap()
			.to_string(),
		ElementType::Generic { r#type, args } => {
			let mut s = String::new();
			write!(&mut s, "{}<", stringify_element_type(metadata, r#type)).unwrap();
			for a in args.iter() {
				write!(&mut s, "{}, ", stringify_element_type(metadata, a)).unwrap();
			}
			s.truncate(s.len() - 2);
			write!(&mut s, ">").unwrap();
			s
		}
		ElementType::SzArray(a) => format!("{}[]", stringify_element_type(metadata, a)),
		_ => format!("{:?}", e),
	}
}
