use super::{PeInfo, RvaAndSize};
use crate::metadata;

use binary_reader::*;

/// ECMA-335 II.25.3.3
#[derive(Debug)]
pub struct CliHeader<'pe> {
	pub pe: &'pe PeInfo<'pe>,
	pub major_runtime_version: u16,
	pub minor_runtime_version: u16,
	pub metadata: RvaAndSize,
	pub flags: u32,
	pub entry_point: metadata::Token,
	pub resources: RvaAndSize,
	pub strong_name_signature: RvaAndSize,
	pub vtable_fixups: RvaAndSize,
}

impl<'pe> CliHeader<'pe> {
	pub(crate) fn from_pe(pe: &'pe PeInfo) -> Option<CliHeader<'pe>> {
		pe.data_directories.get(14).and_then(|&dd| {
			let mut reader = BinaryReader::new(pe.resolve_rva_slice(dd).ok()?);

			let _size = reader.read::<u32>().ok()?;
			let major_runtime_version = reader.read::<u16>().ok()?;
			let minor_runtime_version = reader.read::<u16>().ok()?;
			let metadata = reader.read::<RvaAndSize>().ok()?;
			let flags = reader.read::<u32>().ok()?;
			let entry_point = reader.read::<metadata::Token>().ok()?;
			let resources = reader.read::<RvaAndSize>().ok()?;
			let strong_name_signature = reader.read::<RvaAndSize>().ok()?;
			let _code_manager_table = reader.read::<RvaAndSize>().ok()?;
			let vtable_fixups = reader.read::<RvaAndSize>().ok()?;
			let _export_address_table_jumps = reader.read::<RvaAndSize>().ok()?;
			let _managed_native_header = reader.read::<RvaAndSize>().ok()?;

			Some(CliHeader {
				pe,
				major_runtime_version,
				minor_runtime_version,
				metadata,
				flags,
				entry_point,
				resources,
				strong_name_signature,
				vtable_fixups,
			})
		})
	}

	pub fn metadata(&self) -> Option<metadata::Root<'pe>> {
		crate::metadata::Root::from_data(self.pe.resolve_rva_slice(self.metadata).ok()?)
	}
}
