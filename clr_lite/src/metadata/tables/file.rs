use crate::metadata::*;

#[derive(Debug)]
pub struct File {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileHandle(pub(crate) usize);

impl From<FileHandle> for usize {
	fn from(h: FileHandle) -> usize {
		h.0
	}
}

impl From<usize> for FileHandle {
	fn from(x: usize) -> FileHandle {
		FileHandle(x)
	}
}

impl TableRow for File {
	type Handle = FileHandle;
	const TYPE: TableType = TableType::File;

	fn read_row(reader: &mut TableReader<'_>) -> Result<File, TableReaderError> {
		unimplemented!()
	}
}
