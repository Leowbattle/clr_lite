///! ECMA-335 II.22.23
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct InterfaceImpl {
	pub class: TypeDefHandle,
	pub interface: TypeDefOrRefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceImplHandle(pub(crate) usize);

impl From<InterfaceImplHandle> for usize {
	fn from(h: InterfaceImplHandle) -> usize {
		h.0
	}
}

impl From<usize> for InterfaceImplHandle {
	fn from(x: usize) -> InterfaceImplHandle {
		InterfaceImplHandle(x)
	}
}

impl TableRow for InterfaceImpl {
	type Handle = InterfaceImplHandle;
	const TYPE: TableType = TableType::InterfaceImpl;

	fn read_row(reader: &mut TableReader<'_>) -> Result<InterfaceImpl, TableReaderError> {
		Ok(InterfaceImpl {
			class: reader.read_type_def_handle()?,
			interface: reader.read_type_def_or_ref_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_interface_impl() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/InterfaceImplTests/bin/Debug/netcoreapp3.1/InterfaceImplTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let interface_impls = metadata
			.tables()
			.interface_impl
			.rows()
			.iter()
			.map(|i| {
				(
					metadata
						.strings()
						.get(metadata.tables().type_def[i.class].name)
						.unwrap(),
					metadata
						.strings()
						.get(match i.interface {
							TypeDefOrRefHandle::TypeDefHandle(t) => {
								metadata.tables().type_def[t].name
							}
							TypeDefOrRefHandle::TypeRefHandle(t) => {
								metadata.tables().type_ref[t].name
							}
							_ => unimplemented!(),
						})
						.unwrap(),
				)
			})
			.fold(HashMap::<&str, Vec<&str>>::new(), |mut h, (c, i)| {
				h.entry(c).or_default().push(i);
				h
			});

		assert_eq!(interface_impls["Class1"], &["ABC"]);
		assert_eq!(interface_impls["Class2"], &["DEF", "IDisposable"]);
	}
}
