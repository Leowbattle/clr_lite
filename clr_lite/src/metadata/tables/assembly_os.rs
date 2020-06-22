///! ECMA-335 II.22.3
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct AssemblyOs {
	os_platform_id: u32,
	os_major: u32,
	os_minor: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyOsHandle(pub(crate) usize);

impl From<AssemblyOsHandle> for usize {
	fn from(h: AssemblyOsHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyOsHandle {
	fn from(x: usize) -> AssemblyOsHandle {
		AssemblyOsHandle(x + 1)
	}
}

impl TableRow for AssemblyOs {
	type Handle = AssemblyOsHandle;
	const TYPE: TableType = TableType::AssemblyOs;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyOs, TableReaderError> {
		Ok(AssemblyOs {
			os_platform_id: reader._read::<u32>()?,
			os_major: reader._read::<u32>()?,
			os_minor: reader._read::<u32>()?,
		})
	}
}
