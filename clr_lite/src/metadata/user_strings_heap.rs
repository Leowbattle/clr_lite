use std::fmt;

// ECMA-335 II.24.2.4
pub struct UserStringsHeap<'data> {
	data: &'data [u8],
}

impl fmt::Debug for UserStringsHeap<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("UserStringsHeap").finish()
	}
}

impl<'data> UserStringsHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		Self { data }
	}
}
