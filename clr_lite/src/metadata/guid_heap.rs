pub struct GuidHeap<'data> {
	data: &'data [u8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GuidHandle(usize);

impl GuidHandle {
	fn index(self) -> usize {
		self.0
	}
}

impl From<usize> for GuidHandle {
	fn from(x: usize) -> GuidHandle {
		GuidHandle(x)
	}
}

impl<'data> GuidHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		GuidHeap { data }
	}
}
