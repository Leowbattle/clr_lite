use std::fmt;

/// ECMA-335 II.24.2.5
pub struct GuidHeap<'data> {
	data: &'data [u8],
}

impl fmt::Debug for GuidHeap<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("GuidHeap").finish()
	}
}

def_handle!(GuidHandle);

impl<'data> GuidHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		Self { data }
	}
}
