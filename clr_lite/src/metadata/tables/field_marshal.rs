///! ECMA-335 II.22.17
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct FieldMarshal {
	pub parent: HasFieldMarshalHandle,
	pub native_type: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldMarshalHandle(pub(crate) usize);

impl From<FieldMarshalHandle> for usize {
	fn from(h: FieldMarshalHandle) -> usize {
		h.0
	}
}

impl From<usize> for FieldMarshalHandle {
	fn from(x: usize) -> FieldMarshalHandle {
		FieldMarshalHandle(x + 1)
	}
}

impl TableRow for FieldMarshal {
	type Handle = FieldMarshalHandle;
	const TYPE: TableType = TableType::FieldMarshal;

	fn read_row(reader: &mut TableReader<'_>) -> Result<FieldMarshal, TableReaderError> {
		Ok(FieldMarshal {
			parent: reader.read_has_field_marshal_handle()?,
			native_type: reader.read_blob_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::blob::*;
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_field_marshal() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/FieldMarshalTests/bin/Debug/netcoreapp3.1/FieldMarshalTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let field_marshals = metadata
			.tables()
			.field_marshal
			.rows()
			.iter()
			.map(|f| {
				(
					metadata
						.strings()
						.get(match f.parent {
							HasFieldMarshalHandle::FieldHandle(f) => {
								metadata.tables().field[f].name
							}
							HasFieldMarshalHandle::ParamHandle(p) => {
								metadata.tables().param[p].name
							}
						})
						.unwrap(),
					f,
				)
			})
			.collect::<HashMap<&str, &FieldMarshal>>();

		assert_eq!(
			metadata
				.blob()
				.new_reader(field_marshals["s"].native_type)
				.unwrap()
				.read_marshalling_descriptor()
				.unwrap(),
			MarshallingDescriptor::Scalar(MarshallingDescriptorScalar::String)
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(field_marshals["arr"].native_type)
				.unwrap()
				.read_marshalling_descriptor()
				.unwrap(),
			MarshallingDescriptor::Array {
				element_type: MarshallingDescriptorScalar::Unknown,
				size_param_num: None,
				num_elements: None
			}
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(field_marshals["arr2"].native_type)
				.unwrap()
				.read_marshalling_descriptor()
				.unwrap(),
			MarshallingDescriptor::Array {
				element_type: MarshallingDescriptorScalar::Unknown,
				size_param_num: None,
				num_elements: Some(5)
			}
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(field_marshals["arr3"].native_type)
				.unwrap()
				.read_marshalling_descriptor()
				.unwrap(),
			MarshallingDescriptor::Array {
				element_type: MarshallingDescriptorScalar::Unknown,
				size_param_num: Some(1),
				num_elements: None
			}
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(field_marshals["arr4"].native_type)
				.unwrap()
				.read_marshalling_descriptor()
				.unwrap(),
			MarshallingDescriptor::Array {
				element_type: MarshallingDescriptorScalar::Unknown,
				size_param_num: Some(1),
				num_elements: Some(10)
			}
		);
	}
}
