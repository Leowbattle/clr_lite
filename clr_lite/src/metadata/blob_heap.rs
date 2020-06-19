use crate::metadata::blob::*;

pub struct BlobHeap<'data> {
	data: &'data [u8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BlobHandle(pub(crate) usize);

impl<'data> BlobHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		BlobHeap { data }
	}

	pub fn new_reader(&self, h: BlobHandle) -> Result<BlobReader, BlobReaderError> {
		BlobReader::new(&self.data[h.0..])
	}
}
