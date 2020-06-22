///! ECMA-335 II.22.6
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct AssemblyRefOs {
	os_platform_id: u32,
	os_major: u32,
	os_minor: u32,
	assembly_ref: AssemblyRefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyRefOsHandle(pub(crate) usize);

impl From<AssemblyRefOsHandle> for usize {
	fn from(h: AssemblyRefOsHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyRefOsHandle {
	fn from(x: usize) -> AssemblyRefOsHandle {
		AssemblyRefOsHandle(x + 1)
	}
}

impl TableRow for AssemblyRefOs {
	type Handle = AssemblyRefOsHandle;
	const TYPE: TableType = TableType::AssemblyRefOs;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyRefOs, TableReaderError> {
		Ok(AssemblyRefOs {
			os_platform_id: reader._read::<u32>()?,
			os_major: reader._read::<u32>()?,
			os_minor: reader._read::<u32>()?,
			assembly_ref: reader.read_assembly_ref_handle()?,
		})
	}
}
