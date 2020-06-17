pub struct BlobHeap<'data> {
	data: &'data [u8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BlobHandle(pub(crate) usize);

impl<'data> BlobHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		BlobHeap { data }
	}
}
