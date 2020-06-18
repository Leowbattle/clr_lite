use crate::metadata::*;

#[derive(Debug)]
pub struct GenericParamConstraint {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParamConstraintHandle(pub(crate) usize);

impl From<GenericParamConstraintHandle> for usize {
	fn from(h: GenericParamConstraintHandle) -> usize {
		h.0
	}
}

impl From<usize> for GenericParamConstraintHandle {
	fn from(x: usize) -> GenericParamConstraintHandle {
		GenericParamConstraintHandle(x)
	}
}

impl TableRow for GenericParamConstraint {
	type Handle = GenericParamConstraintHandle;
	const TYPE: TableType = TableType::GenericParamConstraint;

	fn read_row(reader: &mut TableReader<'_>) -> Result<GenericParamConstraint, TableReaderError> {
		unimplemented!()
	}
}
