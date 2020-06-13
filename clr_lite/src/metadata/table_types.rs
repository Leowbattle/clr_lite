use super::HeapSizeFlags;
use num_derive::{FromPrimitive, ToPrimitive};

/// Reference: ECMA-335 §II.22
///
/// "Tables are given both a name (e.g., "Assembly") and a number (e.g., 0x20).  The number for each table is listed immediately with its title in the following subclauses. The table numbers indicate the order in which their corresponding table shall appear in the PE file, and there is a set of bits (§II.24.2.6) saying whether a given table exists or not.  The number of a table is the position within that set of bits."
// I wonder why they chose to make these values bit-indices into a u64 rather than bit flags.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum TableType {
	Module = 0x00,
	TypeRef = 0x01,
	TypeDef = 0x02,
	Field = 0x04,
	MethodDef = 0x06,
	Param = 0x08,
	InterfaceImpl = 0x09,
	MemberRef = 0x0a,
	Constant = 0x0b,
	CustomAttribute = 0x0c,
	FieldMarshal = 0x0d,
	DeclSecurity = 0x0e,
	ClassLayout = 0x0f,
	FieldLayout = 0x10,
	StandaloneSig = 0x11,
	EventMap = 0x12,
	Event = 0x14,
	PropertyMap = 0x15,
	Property = 0x17,
	MethodSemantics = 0x18,
	MethodImpl = 0x19,
	ModuleRef = 0x1a,
	TypeSpec = 0x1b,
	ImplMap = 0x1c,
	FieldRva = 0x1d,
	Assembly = 0x20,
	AssemblyProcessor = 0x21,
	AssemblyOS = 0x22,
	AssemblyRef = 0x23,
	AssemblyRefProcessor = 0x24,
	AssemblyRefOS = 0x25,
	File = 0x26,
	ExportedType = 0x27,
	ManifestResource = 0x28,
	NestedClass = 0x29,
	GenericParam = 0x2a,
	MethodSpec = 0x2b,
	GenericParamConstraint = 0x2c,
}

impl TableType {
	pub fn values() -> impl Iterator<Item = TableType> {
		use TableType::*;
		[
			Module,
			TypeRef,
			TypeDef,
			Field,
			MethodDef,
			Param,
			InterfaceImpl,
			MemberRef,
			Constant,
			CustomAttribute,
			FieldMarshal,
			DeclSecurity,
			ClassLayout,
			FieldLayout,
			StandaloneSig,
			EventMap,
			Event,
			PropertyMap,
			Property,
			MethodSemantics,
			MethodImpl,
			ModuleRef,
			TypeSpec,
			ImplMap,
			FieldRva,
			Assembly,
			AssemblyProcessor,
			AssemblyOS,
			AssemblyRef,
			AssemblyRefProcessor,
			AssemblyRefOS,
			File,
			ExportedType,
			ManifestResource,
			NestedClass,
			GenericParam,
			MethodSpec,
			GenericParamConstraint,
		]
		.iter()
		.copied()
	}
}
