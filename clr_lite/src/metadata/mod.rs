pub mod strings_heap;
pub use strings_heap::*;

pub mod blob_heap;
pub use blob_heap::*;

pub mod blob;

pub mod guid_heap;
pub use guid_heap::*;

pub mod tables;
use tables::*;

pub mod metadata_token;
pub use metadata_token::*;

#[derive(Debug)]
pub enum MetadataError {
	BadImageFormat(String),
}

impl ToString for MetadataError {
	fn to_string(&self) -> String {
		match self {
			MetadataError::BadImageFormat(s) => s.clone(),
		}
	}
}

use crate::pe::*;

pub struct Metadata<'data> {
	pe_data: &'data [u8],
	pe_info: PeInfo<'data>,

	cli_header: CliHeader,

	#[allow(dead_code)]
	metadata: &'data [u8],

	strings_heap: StringsHeap<'data>,
	user_string_data: &'data [u8],
	blob_heap: BlobHeap<'data>,

	#[allow(dead_code)]
	guid_heap: GuidHeap<'data>,
	tables: Tables,

	entry_point: Option<MethodDefHandle>,
}

use binary_reader::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CliHeader {
	size: u32,
	min_major_runtime_version: u16,
	min_minor_runtime_version: u16,
	metadata: RvaAndSize,
	flags: u32,
	entry_point: MetadataToken,
	resources: RvaAndSize,
	strong_name_signature: RvaAndSize,
	code_manager_table: RvaAndSize,
	vtable_fixups: RvaAndSize,
	export_address_table_jumps: RvaAndSize,
	managed_native_header: RvaAndSize,
}

unsafe impl CopyFromBytes for CliHeader {}

#[derive(Copy, Clone, Debug)]
struct StreamHeader<'data> {
	offset: u32,
	size: u32,
	name: &'data str,
}

impl<'data> Metadata<'data> {
	pub fn read(pe_data: &'data [u8]) -> Result<Metadata<'data>, MetadataError> {
		let pe_info =
			PeInfo::parse_pe(pe_data).map_err(|e| MetadataError::BadImageFormat(e.to_string()))?;

		let cli_header = {
			let cli_header_rva_and_size = pe_info
				.data_directories
				.get(14)
				.ok_or_else(|| MetadataError::BadImageFormat("Not .NET assembly".to_string()))?;

			BinaryReader::new(
				&pe_data[pe_info
					.resolve_rva(cli_header_rva_and_size.rva)
					.map_err(|_| {
						MetadataError::BadImageFormat("Invalid CLI header".to_string())
					})?..],
			)
			.read::<CliHeader>()
			.ok_or_else(|| MetadataError::BadImageFormat("Invalid CLI header".to_string()))?
		};

		let metadata = {
			let metadata_offset = pe_info.resolve_rva(cli_header.metadata.rva).map_err(|_| {
				MetadataError::BadImageFormat(format!(
					"Invalid CLI header RVA {}",
					cli_header.metadata.rva
				))
			})?;

			&pe_data[metadata_offset..metadata_offset + cli_header.metadata.size as usize]
		};

		// Parse metadata root
		// ECMA-335 II.24.2.1

		let mut br = BinaryReader::new(metadata);

		if br.read::<u32>() != Some(0x424A5342) {
			return Err(MetadataError::BadImageFormat(
				"Invalid metadata header".to_string(),
			));
		}

		// Skip unused data
		if br.advance(8) == false {
			return Err(MetadataError::BadImageFormat(
				"Invalid metadata header".to_string(),
			));
		}

		let _version = {
			let length = br.read::<u32>().ok_or_else(|| {
				MetadataError::BadImageFormat("Invalid metadata header".to_string())
			})? as usize;
			br.read_str(length).ok_or_else(|| {
				MetadataError::BadImageFormat("Invalid version string in metadata".to_string())
			})?
		};

		// Skip unused flags
		br.advance(2);

		let mut strings_heap = None;
		let mut user_strings_heap = None;
		let mut blob_heap = None;
		let mut guid_heap = None;
		let mut tables = None;

		let number_of_streams = br
			.read::<u16>()
			.ok_or_else(|| MetadataError::BadImageFormat("Invalid metadata header".to_string()))?;

		for _ in 0..number_of_streams {
			let offset = br.read::<u32>().ok_or_else(|| {
				MetadataError::BadImageFormat("Invalid metadata header".to_string())
			})?;
			let size = br.read::<u32>().ok_or_else(|| {
				MetadataError::BadImageFormat("Invalid metadata header".to_string())
			})?;
			let name = br.read_c_str().ok_or_else(|| {
				MetadataError::BadImageFormat("Invalid metadata header".to_string())
			})?;
			br.goto((br.pos() + 4) & !3);

			let header = StreamHeader { offset, size, name };

			match name {
				"#Strings" => strings_heap = Some(header),
				"#US" => user_strings_heap = Some(header),
				"#Blob" => blob_heap = Some(header),
				"#GUID" => guid_heap = Some(header),
				"#~" => tables = Some(header),
				_ => {
					return Err(MetadataError::BadImageFormat(format!(
						"Unrecognised metadata stream \"{}\"",
						name
					)))
				}
			}
		}

		let strings_heap = strings_heap.unwrap();
		let strings_heap = StringsHeap::new(
			&metadata
				[strings_heap.offset as usize..(strings_heap.offset + strings_heap.size) as usize],
		);

		let user_string_data = if let Some(user_strings_heap) = user_strings_heap {
			&metadata[user_strings_heap.offset as usize
				..(user_strings_heap.offset + user_strings_heap.size) as usize]
		} else {
			&[]
		};

		let blob_heap = blob_heap.unwrap();
		let blob_heap = BlobHeap::new(
			&metadata[blob_heap.offset as usize..(blob_heap.offset + blob_heap.size) as usize],
		);

		let guid_heap = guid_heap.unwrap();
		let guid_heap = GuidHeap::new(
			&metadata[guid_heap.offset as usize..(guid_heap.offset + guid_heap.size) as usize],
		);

		let tables = tables.unwrap();
		let tables = TableReader::read(
			&metadata[tables.offset as usize..(tables.offset + tables.size) as usize],
		)
		.map_err(|e| MetadataError::BadImageFormat(e.to_string()))?;

		Ok(Metadata {
			pe_info,
			pe_data,

			cli_header,
			metadata,

			strings_heap,
			user_string_data,
			blob_heap,
			guid_heap,
			tables,

			entry_point: {
				if cli_header.entry_point.index() == 0 {
					None
				} else {
					Some(MethodDefHandle(cli_header.entry_point.index() - 1))
				}
			},
		})
	}

	// pub fn version(&self) -> &'data str {
	// 	self.version
	// }

	pub fn strings(&self) -> &'data StringsHeap {
		&self.strings_heap
	}

	pub fn user_string_data(&self) -> &'data [u8] {
		self.user_string_data
	}

	pub fn blob(&self) -> &'data BlobHeap {
		&self.blob_heap
	}

	pub fn tables<'a>(&'a self) -> &'a Tables {
		&self.tables
	}

	pub fn get_pe_data<'a>(&'a self, rva: RvaAndSize) -> Result<&'a [u8], MetadataError> {
		let offset = self
			.pe_info
			.resolve_rva(rva.rva)
			.map_err(|e| MetadataError::BadImageFormat(e.to_string()))?;
		Ok(&self.pe_data[offset..offset + rva.size as usize])
	}

	pub fn get_resource_data<'a>(&'a self, offset: usize) -> Result<&'a [u8], MetadataError> {
		let data = &self.get_pe_data(self.cli_header.resources)?[offset..];
		let length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
		Ok(&data[4..4 + length])
	}

	pub fn pe_info<'a>(&'a self) -> &'a PeInfo {
		&self.pe_info
	}

	pub fn pe_data<'a>(&'a self) -> &'a [u8] {
		self.pe_data
	}

	pub fn entry_point(&self) -> Option<MethodDefHandle> {
		self.entry_point
	}
}
