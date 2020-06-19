///! ECMA-335 II.22.9
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct Constant {
	pub r#type: ConstantType,
	pub parent: HasConstantHandle,
	pub value: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstantHandle(pub(crate) usize);

impl From<ConstantHandle> for usize {
	fn from(h: ConstantHandle) -> usize {
		h.0
	}
}

impl From<usize> for ConstantHandle {
	fn from(x: usize) -> ConstantHandle {
		ConstantHandle(x + 1)
	}
}

/// In ECMA-335 the constant type is an ElementType, but because it is only allowed certain values I am using
/// a seperate enum here.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConstantType {
	Bool,
	Char,
	SByte,
	Byte,
	Short,
	UShort,
	Int,
	UInt,
	Long,
	ULong,
	Float,
	Double,
	String,
	Null,
}

impl ConstantType {
	fn from_raw(raw: u8) -> Result<ConstantType, TableReaderError> {
		Ok(match raw {
			0x2 => ConstantType::Bool,
			0x3 => ConstantType::Char,
			0x4 => ConstantType::SByte,
			0x5 => ConstantType::Byte,
			0x6 => ConstantType::Short,
			0x7 => ConstantType::UShort,
			0x8 => ConstantType::Int,
			0x9 => ConstantType::UInt,
			0xa => ConstantType::Long,
			0xb => ConstantType::ULong,
			0xc => ConstantType::Float,
			0xd => ConstantType::Double,
			0xe => ConstantType::String,
			0x12 => ConstantType::Null,
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid constant type {}",
					raw
				)))
			}
		})
	}
}

impl TableRow for Constant {
	type Handle = ConstantHandle;
	const TYPE: TableType = TableType::Constant;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Constant, TableReaderError> {
		let r#type = ConstantType::from_raw(reader._read::<u8>()?)?;
		reader.reader.advance(1);
		let parent = reader.read_has_constant_handle()?;
		let value = reader.read_blob_handle()?;

		Ok(Constant {
			r#type,
			parent,
			value,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_constant() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/ConstantTests/bin/Debug/netcoreapp3.1/ConstantTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let constants = metadata
			.tables()
			.constant
			.rows()
			.iter()
			.map(|c| {
				(
					metadata
						.strings()
						.get(match c.parent {
							HasConstantHandle::FieldHandle(f) => metadata.tables().field[f].name,
							_ => unimplemented!(),
						})
						.unwrap(),
					c,
				)
			})
			.collect::<HashMap<&str, &Constant>>();

		assert_eq!(
			metadata
				.blob()
				.new_reader(constants["Bool"].value)
				.unwrap()
				.read::<bool>()
				.unwrap(),
			true
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(constants["Char"].value)
				.unwrap()
				.read::<u16>()
				.unwrap(),
			'A' as u16
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(constants["Int"].value)
				.unwrap()
				.read::<i32>()
				.unwrap(),
			42
		);

		assert_eq!(
			metadata
				.blob()
				.new_reader(constants["Double"].value)
				.unwrap()
				.read::<f64>()
				.unwrap(),
			3.14159
		);

		assert_eq!(
			String::from_utf16(
				metadata
					.blob()
					.new_reader(constants["String"].value)
					.unwrap()
					.read_utf16_str()
					.unwrap()
			)
			.unwrap(),
			"Hello there!".to_string()
		);

		assert_eq!(constants["Null"].r#type, ConstantType::Null);
		assert_eq!(constants["NullString"].r#type, ConstantType::Null);
	}
}
