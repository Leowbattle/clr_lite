use crate::metadata::tables::*;

#[derive(Debug)]
pub struct PropertyMap {
	pub parent: TypeDefHandle,
	pub property_list: PropertyHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PropertyMapHandle(pub(crate) usize);

impl From<PropertyMapHandle> for usize {
	fn from(h: PropertyMapHandle) -> usize {
		h.0
	}
}

impl From<usize> for PropertyMapHandle {
	fn from(x: usize) -> PropertyMapHandle {
		PropertyMapHandle(x + 1)
	}
}

impl TableRow for PropertyMap {
	type Handle = PropertyMapHandle;
	const TYPE: TableType = TableType::PropertyMap;

	fn read_row(reader: &mut TableReader<'_>) -> Result<PropertyMap, TableReaderError> {
		Ok(PropertyMap {
			parent: reader.read_type_def_handle()?,
			property_list: reader.read_property_handle()?,
		})
	}
}
