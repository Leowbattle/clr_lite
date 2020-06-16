#[derive(Debug)]
pub struct InterfaceImpl {
	r#type: TypeDefHandle,
	interface: TypeDefOrRef,
}

crate::def_table!(
	InterfaceImpl,
	InterfaceImplHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<InterfaceImpl> {
		Ok(InterfaceImpl {
			r#type: reader.read_type_def_handle()?,
			interface: reader.read_type_def_or_ref()?,
		})
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pe::*;

	use std::collections::HashMap;

	#[test]
	fn test_interface_impl() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/InterfaceImplTests/bin/Debug/netcoreapp3.1/InterfaceImplTests.dll"
		);

		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		let strings = metadata.strings_heap;
		let type_defs = metadata.tables.type_def;
		let type_refs = metadata.tables.type_ref;

		let interface_impls = metadata.tables.interface_impl.rows().iter().fold(
			HashMap::<&str, Vec<&str>>::new(),
			|mut h, i| {
				h.entry(strings.get(type_defs[i.r#type].type_name).unwrap())
					.or_default()
					.push(
						strings
							.get(match i.interface {
								TypeDefOrRef::TypeDefHandle(t) => type_defs[t].type_name,
								TypeDefOrRef::TypeRefHandle(t) => type_refs[t].type_name,
								_ => unimplemented!(),
							})
							.unwrap(),
					);
				h
			},
		);

		let class1 = interface_impls.get("Class1").unwrap();

		// ECMA-335 says that the InterfaceImpl table is sorted by interface name
		assert_eq!(class1, &["ABC", "DEF", "IDisposable"]);
	}
}
