/// ECMA-335 II.24.2.5
pub struct GuidHeap<'data> {
	data: &'data [u8],
}

def_handle!(GuidHandle);
