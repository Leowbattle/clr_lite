///! ECMA-335 II.22.34
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct Property {
	pub attributes: PropertyAttributes,
	pub name: StringHandle,
	pub signature: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PropertyHandle(pub(crate) usize);

impl From<PropertyHandle> for usize {
	fn from(h: PropertyHandle) -> usize {
		h.0
	}
}

impl From<usize> for PropertyHandle {
	fn from(x: usize) -> PropertyHandle {
		PropertyHandle(x + 1)
	}
}

/// ECMA-335 II.23.1.14
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PropertyAttributes {
	pub special_name: bool,
	pub rt_special_name: bool,
	pub has_default: bool,
}

impl PropertyAttributes {
	pub fn from_raw(raw: u16) -> Result<PropertyAttributes, TableReaderError> {
		Ok(PropertyAttributes {
			special_name: raw & 0x200 == 0x200,
			rt_special_name: raw & 0x400 == 0x400,
			has_default: raw & 0x1000 == 0x1000,
		})
	}
}

impl TableRow for Property {
	type Handle = PropertyHandle;
	const TYPE: TableType = TableType::Property;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Property, TableReaderError> {
		Ok(Property {
			attributes: PropertyAttributes::from_raw(reader._read::<u16>()?)?,
			name: reader.read_string_handle()?,
			signature: reader.read_blob_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_event() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/PropertyTests/bin/Debug/netcoreapp3.1/PropertyTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let properties = metadata
			.tables()
			.property_map
			.rows()
			.iter()
			.enumerate()
			.map(|(i, p)| {
				(
					metadata
						.strings()
						.get(metadata.tables().type_def[p.parent].name)
						.unwrap(),
					{
						let property_end = if i == metadata.tables().property_map.rows().len() - 1 {
							metadata.tables().property.rows().len()
						} else {
							usize::from(metadata.tables().property_map.rows()[i + 1].property_list)
								- 1
						};
						(usize::from(p.property_list) - 1..property_end)
							.map(|p2| {
								metadata
									.strings()
									.get(metadata.tables().property[p2.into()].name)
									.unwrap()
							})
							.collect::<Box<[&str]>>()
					},
				)
			})
			.collect::<HashMap<&str, Box<[&str]>>>();

		assert_eq!(properties["Class1"].as_ref(), &["Property"]);
		assert_eq!(properties["Class3"].as_ref(), &["Prop2", "Prop3"]);
	}
}
