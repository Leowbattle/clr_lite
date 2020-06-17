use crate::metadata::*;

#[derive(Debug)]
pub enum ResolutionScopeHandle {
	Module(ModuleHandle),
	ModuleRef(ModuleRefHandle),
	AssemblyRef(AssemblyRefHandle),
	TypeRef(TypeRefHandle),
}

impl ResolutionScopeHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - ResolutionScopeHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b11;
	pub const TABLES: &'static [TableType] = &[
		TableType::Module,
		TableType::ModuleRef,
		TableType::AssemblyRef,
		TableType::TypeRef,
	];
}
