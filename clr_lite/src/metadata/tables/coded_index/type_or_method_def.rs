use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeOrMethodDefHandle {
	TypeDefHandle(TypeDefHandle),
	MethodDefHandle(MethodDefHandle),
}

impl TypeOrMethodDefHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - TypeOrMethodDefHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b1;
	pub const TABLES: &'static [TableType] = &[TableType::TypeDef, TableType::MethodDef];
}
