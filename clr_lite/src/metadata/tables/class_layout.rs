///! ECMA-335 II.22.8
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct ClassLayout {
	pub packing_size: Option<usize>,
	pub class_size: Option<usize>,
	pub parent: TypeDefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClassLayoutHandle(pub(crate) usize);

impl From<ClassLayoutHandle> for usize {
	fn from(h: ClassLayoutHandle) -> usize {
		h.0
	}
}

impl From<usize> for ClassLayoutHandle {
	fn from(x: usize) -> ClassLayoutHandle {
		ClassLayoutHandle(x + 1)
	}
}

impl TableRow for ClassLayout {
	type Handle = ClassLayoutHandle;
	const TYPE: TableType = TableType::ClassLayout;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ClassLayout, TableReaderError> {
		let packing_size = reader._read::<u16>()? as usize;
		let packing_size = if packing_size == 0 {
			None
		} else {
			Some(packing_size)
		};

		let class_size = reader._read::<u32>()? as usize;
		let class_size = if class_size == 0 {
			None
		} else {
			Some(class_size)
		};

		Ok(ClassLayout {
			packing_size,
			class_size,
			parent: reader.read_type_def_handle()?,
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

		let sized = metadata
			.tables()
			.class_layout
			.rows()
			.iter()
			.map(|l| {
				(
					metadata
						.strings()
						.get(metadata.tables().type_def[l.parent].name)
						.unwrap(),
					l,
				)
			})
			.collect::<HashMap<&str, &ClassLayout>>();

		assert_eq!(sized["Sized"].class_size, Some(42));
		assert_eq!(sized["__StaticArrayInitTypeSize=12"].class_size, Some(12));
	}
}
