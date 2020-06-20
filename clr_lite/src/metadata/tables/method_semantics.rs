///! ECMA-335 II.22.28
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct MethodSemantics {
	pub semantics: MethodSemanticsType,
	pub method: MethodDefHandle,
	pub association: HasSemanticsHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodSemanticsHandle(pub(crate) usize);

impl From<MethodSemanticsHandle> for usize {
	fn from(h: MethodSemanticsHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodSemanticsHandle {
	fn from(x: usize) -> MethodSemanticsHandle {
		MethodSemanticsHandle(x + 1)
	}
}

/// ECMA-335 II.23.1.12
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MethodSemanticsType {
	Setter,
	Getter,
	Other,
	Add,
	Remove,
	Fire,
}

impl MethodSemanticsType {
	fn from_raw(raw: u16) -> Result<MethodSemanticsType, TableReaderError> {
		Ok(match raw {
			0x1 => MethodSemanticsType::Setter,
			0x2 => MethodSemanticsType::Getter,
			0x4 => MethodSemanticsType::Other,
			0x8 => MethodSemanticsType::Add,
			0x10 => MethodSemanticsType::Remove,
			0x20 => MethodSemanticsType::Fire,
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid MethodSemanticsType {:#x}",
					raw
				)))
			}
		})
	}
}

impl TableRow for MethodSemantics {
	type Handle = MethodSemanticsHandle;
	const TYPE: TableType = TableType::MethodSemantics;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodSemantics, TableReaderError> {
		Ok(MethodSemantics {
			semantics: MethodSemanticsType::from_raw(reader._read::<u16>()?)?,
			method: reader.read_method_def_handle()?,
			association: reader.read_has_semantics_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_method_semantics() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/MethodSemanticsTests/bin/Debug/netcoreapp3.1/MethodSemanticsTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let method_semantics = metadata
			.tables()
			.method_semantics
			.rows()
			.iter()
			.map(|m| {
				(
					metadata
						.strings()
						.get(metadata.tables().method_def[m.method].name)
						.unwrap(),
					m,
				)
			})
			.collect::<HashMap<&str, &MethodSemantics>>();

		assert_eq!(
			method_semantics["get_Property"].semantics,
			MethodSemanticsType::Getter
		);

		assert_eq!(
			method_semantics["set_Property"].semantics,
			MethodSemanticsType::Setter
		);
	}
}
