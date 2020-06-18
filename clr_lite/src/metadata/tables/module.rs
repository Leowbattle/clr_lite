use crate::metadata::*;

#[derive(Debug)]
pub struct Module {
	pub name: StringHandle,
	pub version: GuidHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModuleHandle(pub(crate) usize);

impl From<ModuleHandle> for usize {
	fn from(h: ModuleHandle) -> usize {
		h.0
	}
}

impl From<usize> for ModuleHandle {
	fn from(x: usize) -> ModuleHandle {
		ModuleHandle(x)
	}
}

impl TableRow for Module {
	type Handle = ModuleHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Module, TableReaderError> {
		let _generation = reader._read::<u16>();
		let name = reader.read_string_handle()?;
		let version = reader.read_guid_handle()?;
		let _encid = reader.read_guid_handle()?;
		let _encbaseid = reader.read_guid_handle()?;

		Ok(Module { name, version })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	#[test]
	fn test_module() {
		let data =
			include_bytes!("../../../../tests/EmptyExe/bin/Debug/netcoreapp3.1/EmptyExe.dll");
		let metadata = Metadata::read(data).unwrap();

		assert_eq!(
			metadata
				.strings()
				.get(metadata.tables().module[0.into()].name),
			Some("EmptyExe.dll")
		);
	}
}
