///! ECMA-335 II.22.20
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct GenericParam {
	pub index: usize,
	pub attributes: GenericParamAttributes,
	pub owner: TypeOrMethodDefHandle,
	pub name: StringHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParamHandle(pub(crate) usize);

impl From<GenericParamHandle> for usize {
	fn from(h: GenericParamHandle) -> usize {
		h.0
	}
}

impl From<usize> for GenericParamHandle {
	fn from(x: usize) -> GenericParamHandle {
		GenericParamHandle(x + 1)
	}
}

/// ECMA-335 II.23.1.7
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GenericParamAttributes {
	pub variance: GenericParamVariance,
	pub reference_type: bool,
	pub value_type: bool,
	pub default_constructor: bool,
}

impl GenericParamAttributes {
	fn from_raw(raw: u16) -> Result<GenericParamAttributes, TableReaderError> {
		Ok(GenericParamAttributes {
			variance: match raw & 0x3 {
				0x0 => GenericParamVariance::None,
				0x1 => GenericParamVariance::Covariant,
				0x2 => GenericParamVariance::Contravariant,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid generic param variance {}",
						raw & 0x3
					)))
				}
			},
			reference_type: raw & 0x4 == 0x4,
			value_type: raw & 0x8 == 0x8,
			default_constructor: raw & 0x10 == 0x10,
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GenericParamVariance {
	None,
	Covariant,
	Contravariant,
}

impl TableRow for GenericParam {
	type Handle = GenericParamHandle;
	const TYPE: TableType = TableType::GenericParam;

	fn read_row(reader: &mut TableReader<'_>) -> Result<GenericParam, TableReaderError> {
		Ok(GenericParam {
			index: reader._read::<u16>()? as usize,
			attributes: GenericParamAttributes::from_raw(reader._read::<u16>()?)?,
			owner: reader.read_type_or_method_def_handle()?,
			name: reader.read_string_handle()?,
		})
	}
}
