use crate::metadata::tables::*;

#[derive(Debug)]
pub struct FieldLayout {
	pub offset: usize,
	pub field: FieldHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldLayoutHandle(pub(crate) usize);

impl From<FieldLayoutHandle> for usize {
	fn from(h: FieldLayoutHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldLayoutHandle {
	fn from(x: usize) -> FieldLayoutHandle {
		FieldLayoutHandle(x + 1)
	}
}

impl TableRow for FieldLayout {
	type Handle = FieldLayoutHandle;
	const TYPE: TableType = TableType::FieldLayout;

	fn read_row(reader: &mut TableReader<'_>) -> Result<FieldLayout, TableReaderError> {
		Ok(FieldLayout {
			offset: reader._read::<u32>()? as usize,
			field: reader.read_field_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::tables::*;

	use std::collections::HashMap;

	#[test]
	fn test_module() {
		let data =
			include_bytes!("../../../../tests/metadata/tables/ClassLayoutTests/bin/Debug/netcoreapp3.1/ClassLayoutTests.dll");
		let metadata = Metadata::read(data).unwrap();

		let field_layouts = metadata
			.tables()
			.field_layout
			.rows()
			.iter()
			.map(|f| {
				(
					metadata
						.strings()
						.get(metadata.tables().field[f.field].name)
						.unwrap(),
					f,
				)
			})
			.collect::<HashMap<&str, &FieldLayout>>();

		assert_eq!(field_layouts["x"].offset, 12);
	}
}
