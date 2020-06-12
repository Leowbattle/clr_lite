/// ECMA-225 II.24.2.4
pub struct BlobHeap<'data> {
	data: &'data [u8],
}

def_handle!(BlobHandle);
