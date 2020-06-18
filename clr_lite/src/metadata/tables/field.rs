///! ECMA-335 II.22.15
use crate::metadata::*;

#[derive(Debug)]
pub struct Field {
	pub attributes: FieldAttributes,
	pub name: StringHandle,
	pub signature: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldHandle(pub(crate) usize);

impl From<FieldHandle> for usize {
	fn from(h: FieldHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldHandle {
	fn from(x: usize) -> FieldHandle {
		FieldHandle(x)
	}
}

/// ECMA-335 II.23.1.5
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FieldAttributes {
	pub visibility: FieldVisibility,
	pub is_static: bool,
	pub readonly: bool,
	pub is_const: bool,
	pub not_serialised: bool,
	pub special_name: bool,
	pub pinvoke_impl: bool,
	pub rt_special_name: bool,
	pub has_field_marshal: bool,
	pub has_default: bool,
	pub has_field_rva: bool,
}

impl FieldAttributes {
	fn from_raw(raw: u16) -> Result<FieldAttributes, TableReaderError> {
		Ok(FieldAttributes {
			visibility: match raw & 0x7 {
				0x0 => FieldVisibility::CompilerControlled,
				0x1 => FieldVisibility::Private,
				0x2 => FieldVisibility::PrivateProtected,
				0x3 => FieldVisibility::Internal,
				0x4 => FieldVisibility::Protected,
				0x5 => FieldVisibility::ProtectedInternal,
				0x6 => FieldVisibility::Public,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid field visibility {}",
						raw & 0x7
					)))
				}
			},
			is_static: raw & 0x10 == 0x10,
			readonly: raw & 0x20 == 0x20,
			is_const: raw & 0x40 == 0x40,
			not_serialised: raw & 0x80 == 0x80,
			special_name: raw & 0x200 == 0x200,
			pinvoke_impl: raw & 0x2000 == 0x2000,
			rt_special_name: raw & 0x400 == 0x400,
			has_field_marshal: raw & 0x1000 == 0x1000,
			has_default: raw & 0x8000 == 0x8000,
			has_field_rva: raw & 0x100 == 0x100,
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FieldVisibility {
	CompilerControlled,
	Private,
	PrivateProtected,
	Internal,
	Protected,
	ProtectedInternal,
	Public,
}

impl TableRow for Field {
	type Handle = FieldHandle;
	const TYPE: TableType = TableType::Field;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Field, TableReaderError> {
		Ok(Field {
			attributes: FieldAttributes::from_raw(reader._read::<u16>()?)?,
			name: reader.read_string_handle()?,
			signature: reader.read_blob_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	use std::collections::HashMap;

	#[test]
	fn test_field() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/FieldTests/bin/Debug/netcoreapp3.1/FieldTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let fields = metadata
			.tables()
			.field
			.rows()
			.iter()
			.map(|f| (metadata.strings().get(f.name).unwrap(), f))
			.collect::<HashMap<&str, &Field>>();

		assert_eq!(
			fields["Private"].attributes.visibility,
			FieldVisibility::Private
		);

		assert_eq!(
			fields["Public"].attributes.visibility,
			FieldVisibility::Public
		);

		assert!(fields["Static"].attributes.is_static);
		assert!(fields["Readonly"].attributes.readonly);
		assert!(fields["Const"].attributes.is_const);
		assert!(fields["Marshalled"].attributes.has_field_marshal);
	}
}
