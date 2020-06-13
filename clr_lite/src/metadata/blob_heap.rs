use std::fmt;

/// ECMA-225 II.24.2.4
pub struct BlobHeap<'data> {
	data: &'data [u8],
}

impl fmt::Debug for BlobHeap<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("BlobHeap").finish()
	}
}

def_handle!(BlobHandle);

impl<'data> BlobHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		Self { data }
	}
}
