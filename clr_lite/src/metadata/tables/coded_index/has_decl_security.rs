use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HasDeclSecurityHandle {
	TypeDefHandle(TypeDefHandle),
	MethodDefHandle(MethodDefHandle),
	AssemblyHandle(AssemblyHandle),
}

impl HasDeclSecurityHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - HasDeclSecurityHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b11;
	pub const TABLES: &'static [TableType] = &[
		TableType::TypeDef,
		TableType::MethodDef,
		TableType::Assembly,
	];
}
