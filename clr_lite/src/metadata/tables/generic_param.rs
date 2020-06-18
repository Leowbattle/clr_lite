use crate::metadata::*;

#[derive(Debug)]
pub struct GenericParam {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParamHandle(pub(crate) usize);

impl From<GenericParamHandle> for usize {
	fn from(h: GenericParamHandle) -> usize {
		h.0
	}
}

impl From<usize> for GenericParamHandle {
	fn from(x: usize) -> GenericParamHandle {
		GenericParamHandle(x + 1)
	}
}

impl TableRow for GenericParam {
	type Handle = GenericParamHandle;
	const TYPE: TableType = TableType::GenericParam;

	fn read_row(reader: &mut TableReader<'_>) -> Result<GenericParam, TableReaderError> {
		unimplemented!()
	}
}
