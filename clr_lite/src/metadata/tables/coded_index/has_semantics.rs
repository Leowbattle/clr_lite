use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HasSemanticsHandle {
	EventHandle(EventHandle),
	PropertyHandle(PropertyHandle),
}

impl HasSemanticsHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - HasSemanticsHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b1;
	pub const TABLES: &'static [TableType] = &[TableType::Event, TableType::Property];
}
