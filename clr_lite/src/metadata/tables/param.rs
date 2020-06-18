///! ECMA-335 II.22.33
use crate::metadata::*;

#[derive(Debug)]
pub struct Param {
	pub attributes: ParamAttributes,
	pub index: usize,
	pub name: StringHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ParamHandle(pub(crate) usize);

impl From<ParamHandle> for usize {
	fn from(h: ParamHandle) -> usize {
		h.0
	}
}

impl From<usize> for ParamHandle {
	fn from(x: usize) -> ParamHandle {
		ParamHandle(x)
	}
}

/// ECMA-335 II.23.1.13
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParamAttributes {
	pub r#in: bool,
	pub out: bool,
	pub optional: bool,
	pub has_default: bool,
	pub has_field_marshal: bool,
}

impl ParamAttributes {
	fn from_raw(raw: u16) -> ParamAttributes {
		ParamAttributes {
			r#in: raw & 0x1 == 0x1,
			out: raw & 0x2 == 0x2,
			optional: raw & 0x10 == 0x10,
			has_default: raw & 0x1000 == 0x1000,
			has_field_marshal: raw & 0x2000 == 0x2000,
		}
	}
}

impl TableRow for Param {
	type Handle = ParamHandle;
	const TYPE: TableType = TableType::Param;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Param, TableReaderError> {
		Ok(Param {
			attributes: ParamAttributes::from_raw(reader._read::<u16>()?),
			index: reader._read::<u16>()? as usize,
			name: reader.read_string_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_param() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/ParamTests/bin/Debug/netcoreapp3.1/ParamTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();
		let params = metadata
			.tables()
			.param
			.rows()
			.iter()
			.map(|p| metadata.strings().get(p.name).unwrap())
			.collect::<Box<[&str]>>();

		assert!(["x", "y", "z"].iter().all(|s| params.contains(s)));
	}
}
