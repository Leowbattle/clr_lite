///! ECMA-335 II.24.2.6
use super::*;

macro_rules! def_coded_index {
	($name:ident: $($type:ident),*) => {
		pub enum $name {
			$($type($type)),*
		}
	}
}

def_coded_index!(TypeDefOrRef: TypeDefHandle, TypeRefHandle, TypeSpecHandle);
def_coded_index!(HasConstant: FieldHandle, ParamHandle, PropertyHandle);
def_coded_index!(
	HasCustomAttribute: MethodDefHandle,
	FieldHandle,
	TypeRefHandle,
	TypeDefHandle,
	ParamHandle,
	InterfaceImplHandle,
	MemberRefHandle,
	ModuleHandle,
	// PermissionHandle, // TODO Find out what this means, there is no "Permission" table in the spec or mentioned in the dotnet/runtime repo that I could find.
	PropertyHandle,
	EventHandle,
	StandaloneSigHandle,
	ModuleRefHandle,
	TypeSpecHandle,
	AssemblyHandle,
	AssemblyRefHandle,
	FileHandle,
	ExportedTypeHandle,
	ManifestResourceHandle,
	GenericParamHandle,
	GenericParamConstraintHandle,
	MethodSpecHandle
);

def_coded_index!(HasFieldMarshall: FieldHandle, ParamHandle);
def_coded_index!(HasDeclSecurity: TypeDefHandle, MethodDefHandle, AssemblyRef);
def_coded_index!(
	MemberRefParent: TypeDefHandle,
	TypeRefHandle,
	ModuleRefHandle,
	MethodDefHandle,
	TypeSpecHandle
);
def_coded_index!(HasSemantics: EventHandle, PropertyHandle);
def_coded_index!(MethodDefOrRef: MethodDefHandle, MemberRefHandle);
def_coded_index!(MemberForwarded: FieldHandle, MethodDefHandle);
def_coded_index!(
	Implementation: FileHandle,
	AssemblyRefHandle,
	ExportedTypeHandle
);
def_coded_index!(CustomAttributeType: MethodDefHandle, MemberRefHandle);
def_coded_index!(
	ResolutionScope: ModuleHandle,
	ModuleRefHandle,
	AssemblyRefHandle,
	TypeRefHandle
);
def_coded_index!(TypeOrMethodDef: TypeDefHandle, MethodDefHandle);
