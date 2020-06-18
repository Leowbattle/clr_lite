use crate::metadata::*;

#[derive(Debug)]
pub struct Param {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ParamHandle(pub(crate) usize);

impl From<ParamHandle> for usize {
	fn from(h: ParamHandle) -> usize {
		h.0
	}
}

impl From<usize> for ParamHandle {
	fn from(x: usize) -> ParamHandle {
		ParamHandle(x)
	}
}

impl TableRow for Param {
	type Handle = ParamHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Param, TableReaderError> {
		unimplemented!()
	}
}
