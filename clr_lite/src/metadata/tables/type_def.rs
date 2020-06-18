use crate::metadata::*;

#[derive(Debug)]
pub struct TypeDef {
	pub attributes: TypeAttributes,
	pub name: StringHandle,
	pub namespace: StringHandle,
	pub extends: TypeDefOrRefHandle,
	pub field_list: FieldHandle,
	pub method_list: MethodDefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypeDefHandle(pub(crate) usize);

impl From<TypeDefHandle> for usize {
	fn from(h: TypeDefHandle) -> usize {
		h.0
	}
}

impl From<usize> for TypeDefHandle {
	fn from(x: usize) -> TypeDefHandle {
		TypeDefHandle(x)
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl TypeAttributes {
	fn from_raw(raw: u32) -> Result<TypeAttributes, TableReaderError> {
		Ok(TypeAttributes {
			visibility: match raw & 0x7 {
				0x0 => TypeVisibility::NonPublic,
				0x1 => TypeVisibility::Public,
				0x2 => TypeVisibility::NestedPublic,
				0x3 => TypeVisibility::NestedPrivate,
				0x4 => TypeVisibility::NestedProtected,
				0x5 => TypeVisibility::NestedInternal,
				0x6 => TypeVisibility::NestedPrivateProtected,
				0x7 => TypeVisibility::NestedProtectedInternal,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid type visibility {}",
						raw & 0x7
					)))
				}
			},
			layout: match raw & 0x18 {
				0x0 => TypeLayout::Auto,
				0x8 => TypeLayout::Sequential,
				0x10 => TypeLayout::Explicit,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid type layout {}",
						raw & 0x18
					)))
				}
			},
			semantics: match raw & 0x20 {
				0x0 => TypeSemantics::Class,
				0x20 => TypeSemantics::Interface,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid type semantics {}",
						raw & 0x20
					)))
				}
			},
			r#abstract: raw & 0x80 == 0x80,
			sealed: raw & 0x100 == 0x100,
			special_name: raw & 0x400 == 0x400,
			imported: raw & 0x1000 == 0x1000,
			serialisable: raw & 0x2000 == 0x2000,
			string_format: match raw & 0x30000 {
				0x0 => TypeStringFormat::Ansi,
				0x10000 => TypeStringFormat::Unicode,
				0x20000 => TypeStringFormat::Auto,
				0x30000 => TypeStringFormat::Custom,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid type string format {}",
						raw & 0x30000
					)))
				}
			},
			before_field_init: raw & 0x100000 == 0x100000,
			rt_special_name: raw & 0x800 == 0x800,
			has_security: raw & 0x40000 == 0x40000,
			is_type_forwarder: raw & 0x200000 == 0x200000,
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeVisibility {
	NonPublic,
	Public,
	NestedPublic,
	NestedPrivate,
	NestedProtected,
	NestedInternal,
	NestedPrivateProtected,
	NestedProtectedInternal,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeLayout {
	Auto,
	Sequential,
	Explicit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeSemantics {
	Class,
	Interface,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeStringFormat {
	Ansi,
	Unicode,
	Auto,
	Custom,
}

impl TableRow for TypeDef {
	type Handle = TypeDefHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<TypeDef, TableReaderError> {
		Ok(TypeDef {
			attributes: TypeAttributes::from_raw(reader._read::<u32>()?)?,
			name: reader.read_string_handle()?,
			namespace: reader.read_string_handle()?,
			extends: reader.read_type_def_or_ref_handle()?,
			field_list: reader.read_field_handle()?,
			method_list: reader.read_method_def_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	use std::collections::HashMap;

	#[test]
	fn test_type_def() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/TypeDefTests/bin/Debug/netcoreapp3.1/TypeDefTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		struct TypeInfo<'a> {
			def: &'a TypeDef,
			handle: TypeDefHandle,
		}

		let type_defs = metadata
			.tables()
			.type_def
			.rows()
			.iter()
			.enumerate()
			.map(|(i, t)| {
				(
					metadata.strings().get(t.name).unwrap(),
					TypeInfo {
						def: t,
						handle: i.into(),
					},
				)
			})
			.collect::<HashMap<&str, TypeInfo>>();

		assert_eq!(
			type_defs["Class1"].def.attributes.visibility,
			TypeVisibility::Public
		);
		assert_eq!(
			type_defs["Class1"].def.attributes.semantics,
			TypeSemantics::Class
		);

		assert_eq!(
			type_defs["Nested"].def.attributes.visibility,
			TypeVisibility::NestedInternal
		);
		assert_eq!(
			type_defs["Nested"].def.attributes.semantics,
			TypeSemantics::Interface
		);

		assert_eq!(
			type_defs["Subclass"].def.extends,
			TypeDefOrRefHandle::TypeDefHandle(type_defs["Class1"].handle)
		);

		assert!(type_defs["Abstract"].def.attributes.r#abstract);
		assert!(type_defs["Sealed"].def.attributes.sealed);
		assert!(
			!type_defs["NotBeforeFieldInit"]
				.def
				.attributes
				.before_field_init
		);

		assert_eq!(
			type_defs["UnicodeStrings"].def.attributes.string_format,
			TypeStringFormat::Unicode
		);

		assert_eq!(
			type_defs["Explicit"].def.attributes.layout,
			TypeLayout::Explicit
		);
	}
}
