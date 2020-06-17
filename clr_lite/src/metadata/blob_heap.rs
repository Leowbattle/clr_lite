pub struct BlobHeap<'data> {
	data: &'data [u8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BlobHandle(usize);

impl BlobHandle {
	fn index(self) -> usize {
		self.0
	}
}

impl From<usize> for BlobHandle {
	fn from(x: usize) -> BlobHandle {
		BlobHandle(x)
	}
}

impl<'data> BlobHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		BlobHeap { data }
	}
}
