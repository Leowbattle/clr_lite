use std::fmt;

use super::{BlobReader, FieldSig, ReadFieldSig};

// TODO add cache here like in StringsHeap
/// ECMA-225 II.24.2.4
pub struct BlobHeap<'data> {
	pub data: &'data [u8],
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

	pub fn get_field_sig(&self, index: BlobHandle) -> Option<FieldSig> {
		let index = index.into();

		if index > self.data.len() {
			None
		} else {
			BlobReader::new(&self.data[index..]).read_field_sig().ok()
		}
	}
}
