use crate::metadata::tables::*;

#[derive(Debug)]
pub struct ImplMap {
	pub attributes: PInvokeAttributes,
	pub member: MemberForwardedHandle,
	pub name: StringHandle,
	pub module: ModuleRefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplMapHandle(pub(crate) usize);

impl From<ImplMapHandle> for usize {
	fn from(h: ImplMapHandle) -> usize {
		h.0
	}
}

impl From<usize> for ImplMapHandle {
	fn from(x: usize) -> ImplMapHandle {
		ImplMapHandle(x + 1)
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PInvokeAttributes {
	pub no_mangle: bool,
	pub charset: CharSet,
	pub supports_last_error: bool,
	pub calling_convention: CallingConvention,
}

impl PInvokeAttributes {
	fn from_raw(raw: u16) -> Result<PInvokeAttributes, TableReaderError> {
		Ok(PInvokeAttributes {
			no_mangle: raw & 0x1 == 0x1,
			charset: match raw & 0x6 {
				0x0 => CharSet::Unspecified,
				0x2 => CharSet::Ansi,
				0x4 => CharSet::Unicode,
				0x6 => CharSet::Auto,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid charset {}",
						raw & 0x6
					)))
				}
			},
			supports_last_error: raw & 0x40 == 0x40,
			calling_convention: match raw & 0x700 {
				0x100 => CallingConvention::Default,
				0x200 => CallingConvention::CDecl,
				0x300 => CallingConvention::Stdcall,
				0x400 => CallingConvention::Thiscall,
				0x500 => CallingConvention::Fastcall,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid calling convention {}",
						raw & 0x800
					)))
				}
			},
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CharSet {
	Unspecified,
	Ansi,
	Unicode,
	Auto,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CallingConvention {
	Default,
	CDecl,
	Stdcall,
	Thiscall,
	Fastcall,
}

impl TableRow for ImplMap {
	type Handle = ImplMapHandle;
	const TYPE: TableType = TableType::ImplMap;

	fn read_row(reader: &mut TableReader<'_>) -> Result<ImplMap, TableReaderError> {
		Ok(ImplMap {
			attributes: PInvokeAttributes::from_raw(reader._read::<u16>()?)?,
			member: reader.read_member_forwarded_handle()?,
			name: reader.read_string_handle()?,
			module: reader.read_module_ref_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::collections::HashMap;

	#[test]
	fn test_impl_map() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/ImplMapTests/bin/Debug/netcoreapp3.1/ImplMapTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let impl_maps = metadata
			.tables()
			.impl_map
			.rows()
			.iter()
			.map(|i| (metadata.strings().get(i.name).unwrap(), i))
			.collect::<HashMap<_, _>>();

		assert_eq!(
			metadata
				.strings()
				.get(metadata.tables().module_ref[impl_maps["Hello"].module].name)
				.unwrap(),
			"a"
		);

		assert_eq!(
			impl_maps["Hello2"].attributes.calling_convention,
			CallingConvention::CDecl
		);
		assert_eq!(
			metadata
				.strings()
				.get(metadata.tables().module_ref[impl_maps["Hello2"].module].name)
				.unwrap(),
			"b"
		);

		assert_eq!(
			metadata
				.strings()
				.get(metadata.tables().module_ref[impl_maps["Hello3"].module].name)
				.unwrap(),
			"c"
		);
		assert_eq!(impl_maps["Hello3"].attributes.charset, CharSet::Unicode);
	}
}
