use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HasConstantHandle {
	FieldHandle(FieldHandle),
	ParamHandle(ParamHandle),
	PropertyHandle(PropertyHandle),
}

impl HasConstantHandle {
	pub const LARGE_ROW_SIZE: usize = 1 << (16 - HasConstantHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b11;
	pub const TABLES: &'static [TableType] =
		&[TableType::Field, TableType::Param, TableType::Property];
}
