use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemberForwardedHandle {
	FieldHandle(FieldHandle),
	MethodDefHandle(MethodDefHandle),
}

impl MemberForwardedHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - MemberForwardedHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b1;
	pub const TABLES: &'static [TableType] = &[TableType::Field, TableType::MethodDef];
}
