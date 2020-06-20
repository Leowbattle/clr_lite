use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MethodDefOrRefHandle {
	MethodDefHandle(MethodDefHandle),
	MemberRefHandle(MemberRefHandle),
}

impl MethodDefOrRefHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - MethodDefOrRefHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b1;
	pub const TABLES: &'static [TableType] = &[TableType::MethodDef, TableType::MemberRef];
}
