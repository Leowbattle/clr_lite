///! ECMA-335 II.22.27
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct MethodImpl {
	pub class: TypeDefHandle,
	pub body: MethodDefOrRefHandle,
	pub declaration: MethodDefOrRefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodImplHandle(pub(crate) usize);

impl From<MethodImplHandle> for usize {
	fn from(h: MethodImplHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodImplHandle {
	fn from(x: usize) -> MethodImplHandle {
		MethodImplHandle(x + 1)
	}
}

impl TableRow for MethodImpl {
	type Handle = MethodImplHandle;
	const TYPE: TableType = TableType::MethodImpl;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodImpl, TableReaderError> {
		Ok(MethodImpl {
			class: reader.read_type_def_handle()?,
			body: reader.read_method_def_or_ref_handle()?,
			declaration: reader.read_method_def_or_ref_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_method_impl() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/MethodImplTests/bin/Debug/netcoreapp3.1/MethodImplTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let method_impls = metadata
			.tables()
			.method_impl
			.rows()
			.iter()
			.map(|m| {
				(
					metadata
						.strings()
						.get(metadata.tables().type_def[m.class].name)
						.unwrap(),
					metadata
						.strings()
						.get(match m.body {
							MethodDefOrRefHandle::MethodDefHandle(m) => {
								metadata.tables().method_def[m].name
							}
							MethodDefOrRefHandle::MemberRefHandle(m) => {
								metadata.tables().member_ref[m].name
							}
						})
						.unwrap(),
					metadata
						.strings()
						.get(match m.declaration {
							MethodDefOrRefHandle::MethodDefHandle(m) => {
								metadata.tables().method_def[m].name
							}
							MethodDefOrRefHandle::MemberRefHandle(m) => {
								metadata.tables().member_ref[m].name
							}
						})
						.unwrap(),
				)
			})
			.collect::<Box<_>>();

		assert!(method_impls.contains(&("Class1", "MethodImplTests.ABC.Doit", "Doit")));
		assert!(method_impls.contains(&("Class1", "MethodImplTests.DEF.Doit", "Doit")));
	}
}
