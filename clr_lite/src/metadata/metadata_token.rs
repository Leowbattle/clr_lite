use crate::metadata::tables::TableType;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MetadataToken(pub u32);

impl MetadataToken {
	pub fn new(index: usize, table: TableType) -> MetadataToken {
		MetadataToken(index as u32 | ((table as u32) << 24))
	}

	pub fn table(&self) -> usize {
		((self.0 & 0xff000000) >> 24) as usize
	}

	pub fn index(&self) -> usize {
		(self.0 & 0xffffff) as usize
	}
}

impl fmt::Debug for MetadataToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#010x}", self.0)
	}
}

unsafe impl binary_reader::CopyFromBytes for MetadataToken {}
