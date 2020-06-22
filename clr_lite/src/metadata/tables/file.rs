use crate::metadata::tables::*;

#[derive(Debug)]
pub struct File {
	pub contains_metadata: bool,
	pub name: StringHandle,
	pub hash_value: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileHandle(pub(crate) usize);

impl From<FileHandle> for usize {
	fn from(h: FileHandle) -> usize {
		h.0
	}
}

impl From<usize> for FileHandle {
	fn from(x: usize) -> FileHandle {
		FileHandle(x + 1)
	}
}

impl TableRow for File {
	type Handle = FileHandle;
	const TYPE: TableType = TableType::File;

	fn read_row(reader: &mut TableReader<'_>) -> Result<File, TableReaderError> {
		Ok(File {
			contains_metadata: if reader._read::<u32>()? == 0 {
				true
			} else {
				false
			},
			name: reader.read_string_handle()?,
			hash_value: reader.read_blob_handle()?,
		})
	}
}
