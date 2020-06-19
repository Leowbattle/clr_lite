use crate::metadata::tables::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HasCustomAttributeHandle {
	MethodDefHandle(MethodDefHandle),
	FieldHandle(FieldHandle),
	TypeRefHandle(TypeRefHandle),
	TypeDefHandle(TypeDefHandle),
	ParamHandle(ParamHandle),
	InterfaceImplHandle(InterfaceImplHandle),
	MemberRefHandle(MemberRefHandle),
	ModuleHandle(ModuleHandle),
	PropertyHandle(PropertyHandle),
	EventHandle(EventHandle),
	StandaloneSigHandle(StandaloneSigHandle),
	ModuleRefHandle(ModuleRefHandle),
	TypeSpecHandle(TypeSpecHandle),
	AssemblyHandle(AssemblyHandle),
	AssemblyRefHandle(AssemblyRefHandle),
	FileHandle(FileHandle),
	ExportedTypeHandle(ExportedTypeHandle),
	ManifestResourceHandle(ManifestResourceHandle),
	GenericParamHandle(GenericParamHandle),
	GenericParamConstraintHandle(GenericParamConstraintHandle),
	MethodSpecHandle(MethodSpecHandle),
}

impl HasCustomAttributeHandle {
	pub const LARGE_ROW_SIZE: usize =
		1 << (16 - HasCustomAttributeHandle::TAG_MASK.count_ones() as usize);
	pub const TAG_MASK: usize = 0b11111;
	pub const TABLES: &'static [TableType] = &[
		TableType::MethodDef,
		TableType::Field,
		TableType::TypeRef,
		TableType::TypeDef,
		TableType::Param,
		TableType::InterfaceImpl,
		TableType::MemberRef,
		TableType::Module,
		TableType::Property,
		TableType::Event,
		TableType::StandaloneSig,
		TableType::ModuleRef,
		TableType::TypeSpec,
		TableType::Assembly,
		TableType::AssemblyRef,
		TableType::File,
		TableType::ExportedType,
		TableType::ManifestResource,
		TableType::GenericParam,
		TableType::GenericParamConstraint,
		TableType::MethodSpec,
	];
}
