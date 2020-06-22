///! ECMA-335 II.22.2
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct Assembly {
	pub hash_algorithm: AssemblyHashAlgorithm,
	pub major_version: u16,
	pub minor_version: u16,
	pub build_number: u16,
	pub revision_number: u16,
	pub flags: AssemblyFlags,
	pub public_key: BlobHandle,
	pub name: StringHandle,
	pub culture: StringHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AssemblyHandle(pub(crate) usize);

impl From<AssemblyHandle> for usize {
	fn from(h: AssemblyHandle) -> usize {
		h.0
	}
}

impl From<usize> for AssemblyHandle {
	fn from(x: usize) -> AssemblyHandle {
		AssemblyHandle(x + 1)
	}
}

/// ECMA-335 II.23.1.1
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AssemblyHashAlgorithm {
	None,
	Md5,
	Sha1,
}

/// ECMA-335 II.23.1.2
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AssemblyFlags {
	pub public_key: bool,
	pub retargetable: bool,
	pub disable_jit_compiler_optimiser: bool,
	pub enable_jit_compiler_tracking: bool,
}

impl TableRow for Assembly {
	type Handle = AssemblyHandle;
	const TYPE: TableType = TableType::Assembly;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Assembly, TableReaderError> {
		Ok(Assembly {
			hash_algorithm: match reader._read::<u32>()? {
				0x0 => AssemblyHashAlgorithm::None,
				0x8003 => AssemblyHashAlgorithm::Md5,
				0x8004 => AssemblyHashAlgorithm::Sha1,
				_ => {
					return Err(TableReaderError::BadImageFormat(
						"Invalid assembly hash algorithm".to_string(),
					))
				}
			},
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
			public_key: reader.read_blob_handle()?,
			name: reader.read_string_handle()?,
			culture: reader.read_string_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_assembly() {
		let data =
			include_bytes!("../../../../tests/EmptyExe/bin/Debug/netcoreapp3.1/EmptyExe.dll");
		let metadata = Metadata::read(data).unwrap();
		assert_eq!(
			metadata
				.strings()
				.get(metadata.tables().assembly[0.into()].name)
				.unwrap(),
			"EmptyExe"
		);
	}
}
