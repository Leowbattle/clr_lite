use crate::metadata::tables::*;

#[derive(Debug)]
pub struct NestedClass {
	pub nested: TypeDefHandle,
	pub enclosing: TypeDefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NestedClassHandle(pub(crate) usize);

impl From<NestedClassHandle> for usize {
	fn from(h: NestedClassHandle) -> usize {
		h.0
	}
}

impl From<usize> for NestedClassHandle {
	fn from(x: usize) -> NestedClassHandle {
		NestedClassHandle(x + 1)
	}
}

impl TableRow for NestedClass {
	type Handle = NestedClassHandle;
	const TYPE: TableType = TableType::NestedClass;

	fn read_row(reader: &mut TableReader<'_>) -> Result<NestedClass, TableReaderError> {
		Ok(NestedClass {
			nested: reader.read_type_def_handle()?,
			enclosing: reader.read_type_def_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_nested_class() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/TypeDefTests/bin/Debug/netcoreapp3.1/TypeDefTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let nested_classes = metadata
			.tables()
			.nested_class
			.rows()
			.iter()
			.map(|nc| {
				(
					metadata
						.strings()
						.get(metadata.tables().type_def[nc.enclosing].name)
						.unwrap(),
					metadata
						.strings()
						.get(metadata.tables().type_def[nc.nested].name)
						.unwrap(),
				)
			})
			.fold(
				HashMap::<&str, Vec<&str>>::new(),
				|mut h, (enclosing, nested)| {
					h.entry(enclosing).or_default().push(nested);
					h
				},
			);

		assert_eq!(&nested_classes["Class1"], &["Nested"]);
	}
}
