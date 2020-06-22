//! ECMA-335 §II.25.2
//! https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

use std::fmt;

use binary_reader::*;

#[derive(Debug)]
pub enum PeError {
	BadImageFormat(String),
	InvalidRva(Rva),
}

impl ToString for PeError {
	fn to_string(&self) -> String {
		match &self {
			PeError::BadImageFormat(s) => s.clone(),
			PeError::InvalidRva(r) => format!("Invalid RVA {:#x}", r.0),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Rva(pub(crate) u32);

impl fmt::Display for Rva {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#x}", self.0)
	}
}

unsafe impl CopyFromBytes for Rva {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PeHeader {
	machine: u16,
	number_of_sections: u16,
	timestamp: u32,
	symbol_table: u32,
	number_of_symbols: u32,
	optional_header_size: u16,
	characteristics: u16,
}

unsafe impl CopyFromBytes for PeHeader {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pe32OptionalHeader {
	pub magic: u16,
	pub lmajor: u8,
	pub lminor: u8,
	pub code_size: u32,
	pub initialised_data_size: u32,
	pub uninitialised_data_size: u32,
	pub entry_point_rva: u32,
	pub code_rva: u32,
	pub data_rva: u32,

	pub image_base: u32,
	pub section_alignment: u32,
	pub file_alignment: u32,
	pub os_major: u16,
	pub os_minor: u16,
	pub user_major: u16,
	pub user_minor: u16,
	pub subsys_major: u16,
	pub subsys_minor: u16,
	pub reserved: u32,
	pub image_size: u32,
	pub header_size: u32,
	pub checksum: u32,
	pub subsystem: u16,
	pub dll_flags: u16,
	pub stack_reserve_size: u32,
	pub stack_commit_size: u32,
	pub heap_reserve_size: u32,
	pub heap_commit_size: u32,
	pub loader_flags: u32,
	pub number_of_data_directories: u32,
}

unsafe impl CopyFromBytes for Pe32OptionalHeader {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pe64OptionalHeader {
	pub magic: u16,
	pub lmajor: u8,
	pub lminor: u8,
	pub code_size: u32,
	pub initialised_data_size: u32,
	pub uninitialised_data_size: u32,
	pub entry_point_rva: u32,
	pub code_rva: u32,

	pub image_base: u64,
	pub section_alignment: u32,
	pub file_alignment: u32,
	pub os_major: u16,
	pub os_minor: u16,
	pub user_major: u16,
	pub user_minor: u16,
	pub subsys_major: u16,
	pub subsys_minor: u16,
	reserved: u32,
	pub image_size: u32,
	pub header_size: u32,
	pub checksum: u32,
	pub subsystem: u16,
	pub dll_flags: u16,
	pub stack_reserve_size: u64,
	pub stack_commit_size: u64,
	pub heap_reserve_size: u64,
	pub heap_commit_size: u64,
	pub loader_flags: u32,
	pub number_of_data_directories: u32,
}

unsafe impl CopyFromBytes for Pe64OptionalHeader {}

#[derive(Copy, Clone, Debug)]
pub enum PeOptionalHeader {
	Pe32(Pe32OptionalHeader),
	Pe64(Pe64OptionalHeader),
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RvaAndSize {
	pub rva: Rva,
	pub size: u32,
}

unsafe impl CopyFromBytes for RvaAndSize {}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PeSectionHeader {
	name: [u8; 8],
	pub virtual_size: u32,
	pub virtual_address: u32,
	pub size_of_raw_data: u32,
	pub pointer_to_raw_data: u32,
	pub pointer_to_relocations: u32,
	pub pointer_to_line_numbers: u32,
	pub number_of_relocations: u16,
	pub number_of_line_numbers: u16,
	pub characteristics: u32,
}

unsafe impl CopyFromBytes for PeSectionHeader {}

impl PeSectionHeader {
	pub fn name(&self) -> &str {
		std::str::from_utf8(&self.name).unwrap().trim_matches('\0')
	}
}

#[derive(Debug)]
pub struct PeInfo<'data> {
	pub header: PeHeader,
	pub optional_header: PeOptionalHeader,
	pub data_directories: &'data [RvaAndSize],
	pub sections: &'data [PeSectionHeader],
}

impl<'data> PeInfo<'data> {
	pub(crate) fn parse_pe(data: &'data [u8]) -> Result<PeInfo, PeError> {
		let mut br = BinaryReader::new(data);
		if br.peek_array::<u8>(2) != Some(b"MZ") {
			return Err(PeError::BadImageFormat("Not a PE file".to_string()));
		}
		br.goto(0x3c);
		if let None = br.peek::<u32>().map(|offset| br.goto(offset as usize)) {
			return Err(PeError::BadImageFormat(
				"Unexpected end of file".to_string(),
			));
		}
		if br.read_array::<u8>(4) != Some(b"PE\0\0") {
			return Err(PeError::BadImageFormat("Invalid PE file".to_string()));
		}
		let header = br
			.read::<PeHeader>()
			.ok_or_else(|| PeError::BadImageFormat("Invalid PE file".to_string()))?;
		let optional_header = match br
			.peek::<u16>()
			.ok_or_else(|| PeError::BadImageFormat("Invalid PE file".to_string()))?
		{
			0x10b => PeOptionalHeader::Pe32(
				br.read::<Pe32OptionalHeader>()
					.ok_or_else(|| PeError::BadImageFormat("Invalid PE file".to_string()))?,
			),
			0x20b => PeOptionalHeader::Pe64(
				br.read::<Pe64OptionalHeader>()
					.ok_or_else(|| PeError::BadImageFormat("Invalid PE file".to_string()))?,
			),
			_ => return Err(PeError::BadImageFormat("Invalid PE file".to_string())),
		};
		let data_directories = br
			.read_array::<RvaAndSize>(match optional_header {
				PeOptionalHeader::Pe32(p) => p.number_of_data_directories,
				PeOptionalHeader::Pe64(p) => p.number_of_data_directories,
			} as usize)
			.ok_or_else(|| PeError::BadImageFormat("Invalid PE file".to_string()))?;
		let sections = br
			.read_array::<PeSectionHeader>(header.number_of_sections as usize)
			.ok_or_else(|| PeError::BadImageFormat("Invalid PE file".to_string()))?;
		Ok(PeInfo {
			header,
			optional_header,
			data_directories,
			sections,
		})
	}

	/// Returns the file offset of an RVA
	pub fn resolve_rva(&self, rva: Rva) -> Result<usize, PeError> {
		PeInfo::resolve_rva_(rva.0, self.sections)
	}

	fn resolve_rva_(rva: u32, sections: &[PeSectionHeader]) -> Result<usize, PeError> {
		for section in sections {
			if rva >= section.virtual_address
				&& rva < section.virtual_address + section.size_of_raw_data
			{
				return Ok((rva - section.virtual_address + section.pointer_to_raw_data) as usize);
			}
		}
		Err(PeError::BadImageFormat(format!("Invalid RVA {:#x}", rva)))
	}
}
