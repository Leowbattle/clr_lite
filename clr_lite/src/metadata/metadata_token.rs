use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MetadataToken(pub u32);

impl fmt::Debug for MetadataToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#010x}", self.0)
	}
}

unsafe impl binary_reader::CopyFromBytes for MetadataToken {}
