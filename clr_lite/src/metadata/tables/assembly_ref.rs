use crate::metadata::tables::*;

#[derive(Debug)]
pub struct AssemblyRef {
	pub major_version: u16,
	pub minor_version: u16,
	pub build_number: u16,
	pub revision_number: u16,
	pub flags: AssemblyFlags,
	pub public_key_or_token: BlobHandle,
	pub name: StringHandle,
	pub culture: StringHandle,
	pub hash_value: BlobHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyRefHandle(pub(crate) usize);

impl From<AssemblyRefHandle> for usize {
	fn from(h: AssemblyRefHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyRefHandle {
	fn from(x: usize) -> AssemblyRefHandle {
		AssemblyRefHandle(x + 1)
	}
}

impl TableRow for AssemblyRef {
	type Handle = AssemblyRefHandle;
	const TYPE: TableType = TableType::AssemblyRef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<AssemblyRef, TableReaderError> {
		Ok(AssemblyRef {
			major_version: reader._read::<u16>()?,
			minor_version: reader._read::<u16>()?,
			build_number: reader._read::<u16>()?,
			revision_number: reader._read::<u16>()?,
			flags: {
				let flags = reader._read::<u32>()?;
				AssemblyFlags {
					public_key: flags & 0x1 == 0x1,
					retargetable: flags & 0x100 == 0x100,
					disable_jit_compiler_optimiser: flags & 0x4000 == 0x4000,
					enable_jit_compiler_tracking: flags & 0x8000 == 0x8000,
				}
			},
			public_key_or_token: reader.read_blob_handle()?,
			name: reader.read_string_handle()?,
			culture: reader.read_string_handle()?,
			hash_value: reader.read_blob_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_assembly_ref() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/AssemblyRefTests/bin/Debug/netcoreapp3.1/AssemblyRefTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		assert!(metadata
			.tables()
			.assembly_ref
			.rows()
			.iter()
			.map(|r| metadata.strings().get(r.name).unwrap())
			.any(|r| r == "EmptyExe"));
	}
}
