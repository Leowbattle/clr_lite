///! ECMA-335 II.22.10
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct CustomAttribute {
	pub parent: HasCustomAttributeHandle,
	pub attribute_type: CustomAttributeTypeHandle,
	pub value: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CustomAttributeHandle(pub(crate) usize);

impl From<CustomAttributeHandle> for usize {
	fn from(h: CustomAttributeHandle) -> usize {
		h.0
	}
}

impl From<usize> for CustomAttributeHandle {
	fn from(x: usize) -> CustomAttributeHandle {
		CustomAttributeHandle(x + 1)
	}
}

impl TableRow for CustomAttribute {
	type Handle = CustomAttributeHandle;
	const TYPE: TableType = TableType::CustomAttribute;

	fn read_row(reader: &mut TableReader<'_>) -> Result<CustomAttribute, TableReaderError> {
		Ok(CustomAttribute {
			parent: reader.read_has_custom_attribute_handle()?,
			attribute_type: reader.read_custom_attribute_type_handle()?,
			value: reader.read_blob_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_custom_attribute() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/CustomAttributeTests/bin/Debug/netcoreapp3.1/CustomAttributeTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		// Get a list of types and their methods
		let type_defs = metadata
			.tables()
			.type_def
			.rows()
			.iter()
			.enumerate()
			.map(|(i, t)| {
				(metadata.strings().get(t.name).unwrap(), {
					let method_start = t.method_list.into();
					let method_end = if method_start == metadata.tables().method_def.rows().len() {
						metadata.tables().method_def.rows().len()
					} else {
						metadata.tables().type_def[(i + 1).into()]
							.method_list
							.into()
					};
					(method_start..method_end)
						.map(|i| (i - 1).into())
						.collect::<Box<[MethodDefHandle]>>()
				})
			})
			.collect::<Box<[(&str, Box<[MethodDefHandle]>)]>>();

		// Get a list of types and the attributes they implement
		let attribute_defs = metadata
			.tables()
			.custom_attribute
			.rows()
			.iter()
			.map(|c| {
				(
					match c.attribute_type {
						CustomAttributeTypeHandle::MethodDefHandle(m) => {
							type_defs
								.iter()
								.flat_map(|t| t.1.iter().map(move |m2| (t.0, m2)))
								.find(|(_, &m2)| m == m2)
								.unwrap()
								.0
						}
						CustomAttributeTypeHandle::MemberRefHandle(m) => {
							match metadata.tables().member_ref[m].parent {
								MemberRefParentHandle::TypeRefHandle(t) => metadata
									.strings()
									.get(metadata.tables().type_ref[t].name)
									.unwrap(),
								_ => "unimplemented",
							}
						}
					},
					match c.parent {
						HasCustomAttributeHandle::FieldHandle(f) => metadata
							.strings()
							.get(metadata.tables().field[f].name)
							.unwrap(),
						HasCustomAttributeHandle::TypeRefHandle(t) => metadata
							.strings()
							.get(metadata.tables().type_ref[t].name)
							.unwrap(),
						HasCustomAttributeHandle::TypeDefHandle(t) => metadata
							.strings()
							.get(metadata.tables().type_def[t].name)
							.unwrap(),
						HasCustomAttributeHandle::AssemblyHandle(_) => "assembly",
						_ => "unimplemented",
					},
				)
			})
			.fold(HashMap::<&str, Vec<&str>>::new(), |mut h, (a, t)| {
				h.entry(t).or_default().push(a);
				h
			});

		assert!(attribute_defs["MyAttribute"].contains(&"AttributeUsageAttribute"));
		assert!(attribute_defs["Class1"].contains(&"MyAttribute"));
	}
}
