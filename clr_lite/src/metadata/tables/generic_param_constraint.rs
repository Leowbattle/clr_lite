///! ECMA-335 II.22.21
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct GenericParamConstraint {
	pub owner: GenericParamHandle,
	pub constraint: TypeDefOrRefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParamConstraintHandle(pub(crate) usize);

impl From<GenericParamConstraintHandle> for usize {
	fn from(h: GenericParamConstraintHandle) -> usize {
		h.0
	}
}

impl From<usize> for GenericParamConstraintHandle {
	fn from(x: usize) -> GenericParamConstraintHandle {
		GenericParamConstraintHandle(x + 1)
	}
}

impl TableRow for GenericParamConstraint {
	type Handle = GenericParamConstraintHandle;
	const TYPE: TableType = TableType::GenericParamConstraint;

	fn read_row(reader: &mut TableReader<'_>) -> Result<GenericParamConstraint, TableReaderError> {
		Ok(GenericParamConstraint {
			owner: reader.read_generic_param_handle()?,
			constraint: reader.read_type_def_or_ref_handle()?,
		})
	}
}
