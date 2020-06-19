use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemberRefParentHandle {
	TypeDefHandle(TypeDefHandle),
	TypeRefHandle(TypeRefHandle),
	ModuleRefHandle(ModuleRefHandle),
	MethodDefHandle(MethodDefHandle),
	TypeSpecHandle(TypeSpecHandle),
}

impl MemberRefParentHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - MemberRefParentHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b111;
	pub const TABLES: &'static [TableType] = &[
		TableType::TypeDef,
		TableType::TypeRef,
		TableType::ModuleRef,
		TableType::MethodDef,
		TableType::TypeSpec,
	];
}
