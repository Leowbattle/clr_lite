use crate::metadata::*;

#[derive(Debug)]
pub struct ImplMap {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplMapHandle(pub(crate) usize);

impl From<ImplMapHandle> for usize {
	fn from(h: ImplMapHandle) -> usize {
		h.0
	}
}

impl From<usize> for ImplMapHandle {
	fn from(x: usize) -> ImplMapHandle {
		ImplMapHandle(x)
	}
}

impl TableRow for ImplMap {
	type Handle = ImplMapHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ImplMap, TableReaderError> {
		unimplemented!()
	}
}
