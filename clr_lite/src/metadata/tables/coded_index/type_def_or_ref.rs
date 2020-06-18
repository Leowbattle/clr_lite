use crate::metadata::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeDefOrRefHandle {
	TypeDefHandle(TypeDefHandle),
	TypeRefHandle(TypeRefHandle),
	TypeSpecHandle(TypeSpecHandle),
}

impl TypeDefOrRefHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - TypeDefOrRefHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b11;
	pub const TABLES: &'static [TableType] =
		&[TableType::TypeDef, TableType::TypeRef, TableType::TypeSpec];
}
