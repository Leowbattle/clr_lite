use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ImplementationHandle {
	FileHandle(FileHandle),
	AssemblyRefHandle(AssemblyRefHandle),
	ExportedTypeHandle(ExportedTypeHandle),
}

impl ImplementationHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - ImplementationHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b11;
	pub const TABLES: &'static [TableType] = &[
		TableType::File,
		TableType::AssemblyRef,
		TableType::ExportedType,
	];
}
