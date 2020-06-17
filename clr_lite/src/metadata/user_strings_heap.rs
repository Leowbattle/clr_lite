pub struct UserStringsHeap<'data> {
	data: &'data [u8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UserStringHandle(usize);

impl UserStringHandle {
	fn index(self) -> usize {
		self.0
	}
}

impl From<usize> for UserStringHandle {
	fn from(x: usize) -> UserStringHandle {
		UserStringHandle(x)
	}
}

impl<'data> UserStringsHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		UserStringsHeap { data }
	}
}
