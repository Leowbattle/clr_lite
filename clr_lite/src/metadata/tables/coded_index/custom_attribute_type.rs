use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CustomAttributeTypeHandle {
	MethodDefHandle(MethodDefHandle),
	MemberRefHandle(MemberRefHandle),
}

impl CustomAttributeTypeHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - CustomAttributeTypeHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b111;
	pub const TABLES: &'static [TableType] = &[TableType::MethodDef, TableType::MemberRef];
}
