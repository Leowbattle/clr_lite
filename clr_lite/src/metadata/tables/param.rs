#![allow(non_upper_case_globals)]

#[derive(Debug)]
pub struct Param {
	pub flags: ParamAttributes,
	pub index: u16,
	pub name: StringHandle,
}

bitflags! {
	pub struct ParamAttributes: u16 {
		const In = 0x1;
		const Out = 0x2;
		const Optional = 0x10;
		const HasDefault = 0x1000;
		const HasFieldMarshal = 0x2000;
	}
}

crate::def_table!(
	Param,
	ParamHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<Param> {
		let flags = ParamAttributes::from_bits(reader.reader.read::<u16>()?)
			.ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
		let index = reader.reader.read::<u16>()?;
		let name = reader.read_string_handle()?;

		Ok(Param { flags, index, name })
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pe::*;

	use std::collections::HashMap;

	#[test]
	fn test_param() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/ParamTests/bin/Debug/netcoreapp3.1/ParamTests.dll"
		);

		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		let strings = metadata.strings_heap;
		let params = metadata.tables.param;

		let params = params
			.rows()
			.iter()
			.map(|p| (strings.get(p.name.into()).unwrap(), p))
			.collect::<HashMap<&str, &Param>>();

		assert_eq!(params.get("a").unwrap().flags, ParamAttributes::empty());
		assert_eq!(params.get("b").unwrap().flags, ParamAttributes::In);
		assert_eq!(params.get("c").unwrap().flags, ParamAttributes::Out);
		assert_eq!(
			params.get("d").unwrap().flags,
			ParamAttributes::HasDefault | ParamAttributes::Optional
		);
		assert_eq!(
			params.get("e").unwrap().flags,
			ParamAttributes::HasFieldMarshal
		);
	}
}
