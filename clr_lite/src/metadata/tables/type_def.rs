/// ECMA-335 II.22.37
#[derive(Debug, PartialEq, Eq)]
pub struct TypeDef {
	pub flags: TypeAttributes,
	pub type_name: StringHandle,
	pub type_namespace: StringHandle,
	pub extends: TypeDefOrRef,
	pub field_list: FieldHandle,
	pub method_list: MethodDefHandle,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TypeAttributes {
	pub visibility: TypeVisibility,
	pub layout: TypeLayout,
	pub semantics: TypeSemantics,
	pub r#abstract: bool,
	pub sealed: bool,
	pub special_name: bool,
	pub imported: bool,
	pub serialisable: bool,
	pub string_format: TypeStringFormat,
	pub before_field_init: bool,
	pub rt_special_name: bool,
	pub has_security: bool,
	pub is_type_forwarder: bool,
}

impl BinaryReadable for TypeAttributes {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Self> {
		let flags = reader.read::<u32>()?;

		let visibility = match flags & 0x7 {
			0x0 => TypeVisibility::NonPublic,
			0x1 => TypeVisibility::Public,
			0x2 => TypeVisibility::NestedPublic,
			0x3 => TypeVisibility::NestedPrivate,
			0x4 => TypeVisibility::NestedFamily,
			0x5 => TypeVisibility::NestedAssembly,
			0x6 => TypeVisibility::NestedFamAndAsm,
			0x7 => TypeVisibility::NestedFamOrAsm,
			_ => return Err(io::Error::from(io::ErrorKind::InvalidData)),
		};

		let layout = match flags & 0x18 {
			0x0 => TypeLayout::AutoLayout,
			0x8 => TypeLayout::SequentialLayout,
			0x10 => TypeLayout::ExplicitLayout,
			_ => return Err(io::Error::from(io::ErrorKind::InvalidData)),
		};

		let semantics = match flags & 0x20 {
			0x0 => TypeSemantics::Class,
			0x20 => TypeSemantics::Interface,
			_ => return Err(io::Error::from(io::ErrorKind::InvalidData)),
		};

		let r#abstract = flags & 0x80 != 0;
		let sealed = flags & 0x100 != 0;
		let special_name = flags & 0x400 != 0;

		let imported = flags & 0x1000 != 0;
		let serialisable = flags & 0x2000 != 0;

		let string_format = match flags & 0x30000 {
			0x0 => TypeStringFormat::Ansi,
			0x10000 => TypeStringFormat::Unicode,
			0x20000 => TypeStringFormat::Auto,
			0x30000 => TypeStringFormat::Custom,
			_ => return Err(io::Error::from(io::ErrorKind::InvalidData)),
		};

		let before_field_init = flags & 0x100000 != 0;

		let rt_special_name = flags & 0x800 != 0;
		let has_security = flags & 0x40000 != 0;
		let is_type_forwarder = flags & 0x200000 != 0;

		Ok(TypeAttributes {
			visibility,
			layout,
			semantics,
			r#abstract,
			sealed,
			special_name,
			imported,
			serialisable,
			string_format,
			before_field_init,
			rt_special_name,
			has_security,
			is_type_forwarder,
		})
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeVisibility {
	NonPublic,
	Public,
	NestedPublic,
	NestedPrivate,
	NestedFamily,
	NestedAssembly,
	NestedFamAndAsm,
	NestedFamOrAsm,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeLayout {
	AutoLayout,
	SequentialLayout,
	ExplicitLayout,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeSemantics {
	Class,
	Interface,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeStringFormat {
	Ansi,
	Unicode,
	Auto,
	Custom,
}

crate::def_table!(
	TypeDef,
	TypeDefHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<TypeDef> {
		let flags = reader.reader.read::<TypeAttributes>()?;

		let type_name = reader.read_string_handle()?;
		let type_namespace = reader.read_string_handle()?;
		let extends = reader.read_type_def_or_ref()?;

		let field_list = reader.read_field_handle()?;
		let method_list = reader.read_method_def_handle()?;

		Ok(TypeDef {
			flags,
			type_name,
			type_namespace,
			extends,
			field_list,
			method_list,
		})
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pe::*;

	use std::collections::HashMap;

	#[test]
	fn test_type_def() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/TypeRefTests/bin/Debug/netcoreapp3.1/TypeRefTests.dll"
		);
		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		let strings = metadata.strings_heap;
		let types = metadata.tables.type_def.as_ref().unwrap();

		#[derive(Debug)]
		struct TypeInfo<'a> {
			i: usize,
			row: &'a TypeDef,
			field_count: usize,
			method_count: usize,
		}

		let types = types
			.rows()
			.iter()
			.enumerate()
			.map(|(i, row)| {
				(
					strings.get(row.type_name).unwrap(),
					TypeInfo {
						i,
						row,
						// The field count is the field index of the next type - the field index of this type, or the number of fields - the field index of this type if this type is the last one.
						field_count: if i == types.rows().len() - 1 {
							// TODO Implement field table
							0
						} else {
							types.rows()[i + 1].field_list.0 - row.field_list.0
						},
						method_count: if i == types.rows().len() - 1 {
							// TODO Implement method table
							0
						} else {
							types.rows()[i + 1].method_list.0 - row.method_list.0
						},
					},
				)
			})
			.collect::<HashMap<&str, TypeInfo>>();

		let module = types.get("<Module>").unwrap();
		assert_eq!(module.row.flags.visibility, TypeVisibility::NonPublic);

		let class1 = types.get("Class1").unwrap();
		assert_eq!(class1.row.flags.visibility, TypeVisibility::Public);
		assert_eq!(class1.field_count, 3); // Class1 has 3 fields: x, y, and z.
		assert_eq!(class1.method_count, 2); // Class1 has 2 methods: .ctor and Doit

		// TODO Maybe I should write a proper test framework to check this stuff more easily

		let subclass = types.get("Subclass").unwrap();
		assert_eq!(
			subclass.row.extends,
			TypeDefOrRef::TypeDefHandle((class1.i + 1).into())
		); // class Subclass : Class1 {}

		let extends_external = types.get("ExtendsExternalType").unwrap();
		assert_eq!(
			extends_external.row.extends,
			TypeDefOrRef::TypeRefHandle(
				(metadata
					.tables
					.type_ref
					.as_ref()
					.unwrap()
					.rows()
					.iter()
					.position(|r| strings.get(r.type_name) == Some("Exception"))
					.unwrap() + 1)
					.into()
			)
		); // class ExtendsExternalType : Exception { }

		let sealed = types.get("SealedClass").unwrap();
		assert_eq!(sealed.row.flags.sealed, true);

		let r#abstract = types.get("AbstractClass").unwrap();
		assert_eq!(r#abstract.row.flags.r#abstract, true);

		let interface = types.get("Interface").unwrap();
		assert_eq!(interface.row.flags.semantics, TypeSemantics::Interface);
		assert_eq!(interface.row.flags.r#abstract, true);

		let r#struct = types.get("Struct").unwrap();
		assert_eq!(
			r#struct.row.extends,
			TypeDefOrRef::TypeRefHandle(
				(metadata
					.tables
					.type_ref
					.as_ref()
					.unwrap()
					.rows()
					.iter()
					.position(|r| strings.get(r.type_name) == Some("ValueType"))
					.unwrap() + 1)
					.into()
			)
		);

		let explicit_layout = types.get("ExplicitLayoutStruct").unwrap();
		assert_eq!(explicit_layout.row.flags.layout, TypeLayout::ExplicitLayout);

		let special_name = types.get("SpecialName").unwrap();
		assert_eq!(special_name.row.flags.special_name, true);

		let serialisable_class = types.get("SerialisableClass").unwrap();
		assert_eq!(serialisable_class.row.flags.serialisable, true);

		let unicode_struct = types.get("UnicodeStruct").unwrap();
		assert_eq!(
			unicode_struct.row.flags.string_format,
			TypeStringFormat::Unicode
		);

		let auto_struct = types.get("AutoStruct").unwrap();
		assert_eq!(auto_struct.row.flags.string_format, TypeStringFormat::Auto);

		let has_static_constructor = types.get("HasStaticConstructor").unwrap();
		assert_eq!(has_static_constructor.row.flags.before_field_init, false);
	}
}
