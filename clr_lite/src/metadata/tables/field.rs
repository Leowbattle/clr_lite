#![allow(non_upper_case_globals)]

// ECMA-335 II.22.15
#[derive(Debug, PartialEq, Eq)]
pub struct Field {
	pub flags: FieldAttributes,
	pub name: StringHandle,
	pub signature: BlobHandle,
}

bitflags! {
	/// Note: The fmt::Debug implementation is wrong, but there is no way to override it
	pub struct FieldAttributes: u16 {
		const VisibilityMask = 0x7;
		const CompilerControlled = 0x0;
		const Private = 0x1;
		const PrivateProtected = 0x2;
		const Internal = 0x3;
		const Protected = 0x4;
		const ProtectedInternal = 0x5;
		const Public = 0x6;

		const Static = 0x10;
		const InitOnly = 0x20;
		const Literal = 0x40;
		const NotSerialised = 0x80;
		const SpecialName = 0x200;

		const PInvokeImpl = 0x2000;

		const RtSpecialName = 0x400;
		const HasFieldMarshal = 0x1000;
		const HasDefault = 0x8000;
		const HasFieldRva = 0x100;
	}
}

crate::def_table!(
	Field,
	FieldHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<Field> {
		let flags = FieldAttributes::from_bits(reader.reader.read::<u16>()?)
			.ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
		let name = reader.read_string_handle()?;
		let signature = reader.read_blob_handle()?;

		Ok(Field {
			flags,
			name,
			signature,
		})
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pe::*;

	use std::collections::HashMap;

	#[test]
	fn test_field() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/FieldTests/bin/Debug/netcoreapp3.1/FieldTests.dll"
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
						field_count: if row.field_list.0 == fields.rows().len() + 1 {
							0
						} else if i == types.rows().len() - 1 {
							fields.rows().len() - row.field_list.0
						} else {
							types.rows()[i + 1].field_list.0 - row.field_list.0
						},
					},
				)
			})
			.collect::<HashMap<&str, TypeInfo>>();

		let fields = fields
			.rows()
			.iter()
			.map(|f| (strings.get(f.name.into()).unwrap(), f))
			.collect::<HashMap<&str, &Field>>();

		let class1 = types.get("Class1").unwrap();
		assert_eq!(class1.field_count, 7);

		let class2 = types.get("Class2").unwrap();
		assert_eq!(class2.field_count, 0);

		let fruit = types.get("Fruit").unwrap();
		assert_eq!(fruit.field_count, 5); // 4 variants + value__

		let my_static = fields.get("MyStatic").unwrap();
		assert!(my_static.flags.contains(FieldAttributes::Static));

		let my_readonly = fields.get("MyReadonly").unwrap();
		assert!(my_readonly.flags.contains(FieldAttributes::InitOnly));

		let my_const = fields.get("MyConst").unwrap();
		assert!(my_const.flags.contains(
			FieldAttributes::Static | FieldAttributes::Literal | FieldAttributes::HasDefault
		));
	}
}
