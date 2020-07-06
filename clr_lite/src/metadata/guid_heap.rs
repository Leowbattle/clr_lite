pub struct GuidHeap<'data> {
	_data: &'data [u8],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GuidHandle(pub(crate) usize);

impl<'data> GuidHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		GuidHeap { _data: data }
	}
}
