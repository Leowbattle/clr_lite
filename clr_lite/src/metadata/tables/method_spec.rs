///! ECMA-335 II.22.29
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct MethodSpec {
	pub method: MethodDefOrRefHandle,
	pub instantiation: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodSpecHandle(pub(crate) usize);

impl From<MethodSpecHandle> for usize {
	fn from(h: MethodSpecHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodSpecHandle {
	fn from(x: usize) -> MethodSpecHandle {
		MethodSpecHandle(x + 1)
	}
}

impl TableRow for MethodSpec {
	type Handle = MethodSpecHandle;
	const TYPE: TableType = TableType::MethodSpec;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodSpec, TableReaderError> {
		Ok(MethodSpec {
			method: reader.read_method_def_or_ref_handle()?,
			instantiation: reader.read_blob_handle()?,
		})
	}
}
