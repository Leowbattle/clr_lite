#![allow(non_upper_case_globals)]

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

bitflags! {
	/// Note: The fmt::Debug implementation is wrong, but there is no way to override it
	pub struct TypeAttributes: u32 {
		const VisibilityMask = 0x7;
		const NonPublic = 0x0;
		const Public = 0x1;
		const NestedPublic = 0x2;
		const NestedPrivate = 0x3;
		const NestedProtected = 0x4;
		const NestedInternal = 0x5;
		const NestedPrivateProtected = 0x6;
		const NestedProtectedInternal = 0x7;

		const LayoutMask = 0x18;
		const AutoLayout = 0x0;
		const SequentialLayout = 0x8;
		const ExplicitLayout = 0x10;

		const SemanticsMask = 0x20;
		const Class = 0x0;
		const Interface = 0x20;

		const Abstract = 0x80;
		const Sealed = 0x100;
		const SpecialName = 0x400;

		const Imported = 0x1000;
		const Serialisable = 0x2000;

		const StringFormatMask = 0x30000;
		const Ansi = 0x0;
		const Unicode = 0x10000;
		const Auto = 0x20000;
		const Custom = 0x30000;

		const BeforeFieldInit = 0x100000;

		const RtSpecialName = 0x800;
		const HasSecurity = 0x40000;
		const IsTypeForwarder = 0x200000;
	}
}

crate::def_table!(
	TypeDef,
	TypeDefHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<TypeDef> {
		let flags = TypeAttributes::from_bits(reader.reader.read::<u32>()?)
			.ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

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
			"../../../../tests/metadata/tables/TypeDefTests/bin/Debug/netcoreapp3.1/TypeDefTests.dll"
		);
		let pe = PeInfo::parse(data).unwrap();
		let cli_header = pe.cli_header();
		let metadata = cli_header.and_then(|c| c.metadata()).unwrap();

		let strings = metadata.strings_heap;
		let types = &metadata.tables.type_def;
		let fields = &metadata.tables.field;

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
						field_count: if row.field_list.0 == fields.rows().len() + 1 {
							0
						} else if i == types.rows().len() - 1 {
							fields.rows().len() - row.field_list.0
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
		assert!(
			(module.row.flags & TypeAttributes::VisibilityMask).contains(TypeAttributes::NonPublic)
		);

		let class1 = types.get("Class1").unwrap();
		assert!(
			(class1.row.flags & TypeAttributes::VisibilityMask).contains(TypeAttributes::Public)
		);
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
					.rows()
					.iter()
					.position(|r| strings.get(r.type_name) == Some("Exception"))
					.unwrap() + 1)
					.into()
			)
		); // class ExtendsExternalType : Exception { }

		let sealed = types.get("SealedClass").unwrap();
		assert!(sealed.row.flags.contains(TypeAttributes::Sealed));

		let r#abstract = types.get("AbstractClass").unwrap();
		assert!(r#abstract.row.flags.contains(TypeAttributes::Abstract));

		let interface = types.get("Interface").unwrap();
		assert!((interface.row.flags & TypeAttributes::SemanticsMask)
			.contains(TypeAttributes::Interface));
		assert!(interface.row.flags.contains(TypeAttributes::Abstract));

		let r#struct = types.get("Struct").unwrap();
		assert_eq!(
			r#struct.row.extends,
			TypeDefOrRef::TypeRefHandle(
				(metadata
					.tables
					.type_ref
					.rows()
					.iter()
					.position(|r| strings.get(r.type_name) == Some("ValueType"))
					.unwrap() + 1)
					.into()
			)
		);

		let explicit_layout = types.get("ExplicitLayoutStruct").unwrap();
		assert!((explicit_layout.row.flags & TypeAttributes::LayoutMask)
			.contains(TypeAttributes::ExplicitLayout));

		let special_name = types.get("SpecialName").unwrap();
		assert!(special_name.row.flags.contains(TypeAttributes::SpecialName));

		let serialisable_class = types.get("SerialisableClass").unwrap();
		assert!(serialisable_class
			.row
			.flags
			.contains(TypeAttributes::Serialisable));

		let unicode_struct = types.get("UnicodeStruct").unwrap();
		assert!(
			(unicode_struct.row.flags & TypeAttributes::StringFormatMask)
				.contains(TypeAttributes::Unicode)
		);

		let auto_struct = types.get("AutoStruct").unwrap();
		assert!((auto_struct.row.flags & TypeAttributes::StringFormatMask)
			.contains(TypeAttributes::Auto));

		let has_static_constructor = types.get("HasStaticConstructor").unwrap();
		assert!(!has_static_constructor
			.row
			.flags
			.contains(TypeAttributes::BeforeFieldInit));
	}
}
