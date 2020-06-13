/// ECMA-335 II.25.1
///! https://docs.microsoft.com/en-us/windows/win32/debug/pe-format
use std::fmt;
use std::io::{self, Seek};
use std::result;

use binary_reader::*;

pub mod cli_header;
pub use cli_header::*;

#[derive(Copy, Clone, Debug)]
pub enum PeError {
	NotPe,
	InvalidHeader,
	InvalidRva,
}

pub type Result<T> = result::Result<T, PeError>;

#[derive(Copy, Clone, Debug)]
pub struct Rva(u32);

impl BinaryReadable for Rva {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Rva> {
		Ok(Rva(reader.read::<u32>()?))
	}
}

pub struct PeInfo<'data> {
	data: &'data [u8],
	pub header: PeHeader,
	pub optional_header: PeOptionalHeader,
	pub data_directories: Box<[RvaAndSize]>,
	pub sections: Box<[PeSectionHeader]>,
}

impl fmt::Debug for PeInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PeInfo")
			.field("header", &self.header)
			.field("optional_header", &self.optional_header)
			.field("data_directories", &self.data_directories)
			.field("sections", &self.sections)
			.finish()
	}
}

#[derive(Copy, Clone, Debug)]
/// ECMA-335 II.25.2.2
/// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#coff-file-header-object-and-image
pub struct PeHeader {
	pub machine: u16,
	pub number_of_sections: u16,
	pub timestamp: u32,
	pub pointer_to_symbol_table: u32,
	pub number_of_symbols: u32,
	pub optional_header_size: u16,
	pub characteristics: u16,
}

impl BinaryReadable for PeHeader {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<PeHeader> {
		Ok(PeHeader {
			machine: reader.read::<u16>()?,
			number_of_sections: reader.read::<u16>()?,
			timestamp: reader.read::<u32>()?,
			pointer_to_symbol_table: reader.read::<u32>()?,
			number_of_symbols: reader.read::<u32>()?,
			optional_header_size: reader.read::<u16>()?,
			characteristics: reader.read::<u16>()?,
		})
	}
}

#[derive(Copy, Clone, Debug)]
/// ECMA-335 II.25.2.3
/// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#optional-header-windows-specific-fields-image-only
pub enum PeOptionalHeader {
	Pe32(Pe32OptionalHeader),
	Pe64(Pe64OptionalHeader),
}

#[derive(Copy, Clone, Debug)]
pub struct Pe32OptionalHeader {
	pub magic: u16,
	pub major_linker_version: u8,
	pub minor_linker_version: u8,
	pub size_of_code: u32,
	pub size_of_initialised_data: u32,
	pub size_of_uninitialised_data: u32,
	pub address_of_entry_point: Rva,
	pub code_rva: Rva,
	pub data_rva: Rva,

	pub image_base: u32,
	pub section_alignment: u32,
	pub file_alignment: u32,
	pub major_os_version: u16,
	pub minor_os_version: u16,
	pub major_image_version: u16,
	pub minor_image_version: u16,
	pub major_subsystem_version: u16,
	pub minor_subsystem_version: u16,
	reserved: u32,
	pub size_of_image: u32,
	pub size_of_headers: u32,
	pub checksum: u32,
	pub subsystem: u16,
	pub dll_characteristics: u16,
	pub stack_reserve: u32,
	pub stack_commit: u32,
	pub heap_reserve: u32,
	pub heap_commit: u32,
	reserved2: u32,
	pub number_of_data_directories: u32,
}

impl BinaryReadable for Pe32OptionalHeader {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Pe32OptionalHeader> {
		Ok(Pe32OptionalHeader {
			magic: reader.read::<u16>()?,
			major_linker_version: reader.read::<u8>()?,
			minor_linker_version: reader.read::<u8>()?,
			size_of_code: reader.read::<u32>()?,
			size_of_initialised_data: reader.read::<u32>()?,
			size_of_uninitialised_data: reader.read::<u32>()?,
			address_of_entry_point: reader.read::<Rva>()?,
			code_rva: reader.read::<Rva>()?,
			data_rva: reader.read::<Rva>()?,

			image_base: reader.read::<u32>()?,
			section_alignment: reader.read::<u32>()?,
			file_alignment: reader.read::<u32>()?,
			major_os_version: reader.read::<u16>()?,
			minor_os_version: reader.read::<u16>()?,
			major_image_version: reader.read::<u16>()?,
			minor_image_version: reader.read::<u16>()?,
			major_subsystem_version: reader.read::<u16>()?,
			minor_subsystem_version: reader.read::<u16>()?,
			reserved: reader.read::<u32>()?,
			size_of_image: reader.read::<u32>()?,
			size_of_headers: reader.read::<u32>()?,
			checksum: reader.read::<u32>()?,
			subsystem: reader.read::<u16>()?,
			dll_characteristics: reader.read::<u16>()?,
			stack_reserve: reader.read::<u32>()?,
			stack_commit: reader.read::<u32>()?,
			heap_reserve: reader.read::<u32>()?,
			heap_commit: reader.read::<u32>()?,
			reserved2: reader.read::<u32>()?,
			number_of_data_directories: reader.read::<u32>()?,
		})
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Pe64OptionalHeader {
	pub magic: u16,
	pub major_linker_version: u8,
	pub minor_linker_version: u8,
	pub size_of_code: u32,
	pub size_of_initialised_data: u32,
	pub size_of_uninitialised_data: u32,
	pub address_of_entry_point: Rva,
	pub code_rva: Rva,

	pub image_base: u64,
	pub section_alignment: u32,
	pub file_alignment: u32,
	pub major_os_version: u16,
	pub minor_os_version: u16,
	pub major_image_version: u16,
	pub minor_image_version: u16,
	pub major_subsystem_version: u16,
	pub minor_subsystem_version: u16,
	reserved: u32,
	pub size_of_image: u32,
	pub size_of_headers: u32,
	pub checksum: u32,
	pub subsystem: u16,
	pub dll_characteristics: u16,
	pub stack_reserve: u64,
	pub stack_commit: u64,
	pub heap_reserve: u64,
	pub heap_commit: u64,
	reserved2: u32,
	pub number_of_data_directories: u32,
}

impl BinaryReadable for Pe64OptionalHeader {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Pe64OptionalHeader> {
		Ok(Pe64OptionalHeader {
			magic: reader.read::<u16>()?,
			major_linker_version: reader.read::<u8>()?,
			minor_linker_version: reader.read::<u8>()?,
			size_of_code: reader.read::<u32>()?,
			size_of_initialised_data: reader.read::<u32>()?,
			size_of_uninitialised_data: reader.read::<u32>()?,
			address_of_entry_point: reader.read::<Rva>()?,
			code_rva: reader.read::<Rva>()?,

			image_base: reader.read::<u64>()?,
			section_alignment: reader.read::<u32>()?,
			file_alignment: reader.read::<u32>()?,
			major_os_version: reader.read::<u16>()?,
			minor_os_version: reader.read::<u16>()?,
			major_image_version: reader.read::<u16>()?,
			minor_image_version: reader.read::<u16>()?,
			major_subsystem_version: reader.read::<u16>()?,
			minor_subsystem_version: reader.read::<u16>()?,
			reserved: reader.read::<u32>()?,
			size_of_image: reader.read::<u32>()?,
			size_of_headers: reader.read::<u32>()?,
			checksum: reader.read::<u32>()?,
			subsystem: reader.read::<u16>()?,
			dll_characteristics: reader.read::<u16>()?,
			stack_reserve: reader.read::<u64>()?,
			stack_commit: reader.read::<u64>()?,
			heap_reserve: reader.read::<u64>()?,
			heap_commit: reader.read::<u64>()?,
			reserved2: reader.read::<u32>()?,
			number_of_data_directories: reader.read::<u32>()?,
		})
	}
}

impl BinaryReadable for PeOptionalHeader {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<PeOptionalHeader> {
		let magic = reader.peek::<u16>()?;
		Ok(match magic {
			0x10b => PeOptionalHeader::Pe32(reader.read::<Pe32OptionalHeader>()?),
			0x20b => PeOptionalHeader::Pe64(reader.read::<Pe64OptionalHeader>()?),
			_ => return Err(io::Error::from(io::ErrorKind::Other)),
		})
	}
}

#[derive(Copy, Clone, Debug)]
pub struct RvaAndSize {
	pub rva: Rva,
	pub size: u32,
}

impl BinaryReadable for RvaAndSize {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<RvaAndSize> {
		Ok(RvaAndSize {
			rva: reader.read::<Rva>()?,
			size: reader.read::<u32>()?,
		})
	}
}

#[derive(Clone, Debug)]
pub struct PeSectionHeader {
	pub name: String,
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

impl BinaryReadable for PeSectionHeader {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<PeSectionHeader> {
		let mut name = [0; 8];
		reader.read_slice(&mut name)?;
		Ok(PeSectionHeader {
			name: String::from(
				std::str::from_utf8(&name)
					.map_err(|_| io::Error::from(io::ErrorKind::Other))?
					.trim_matches('\0'),
			),
			virtual_size: reader.read::<u32>()?,
			virtual_address: reader.read::<u32>()?,
			size_of_raw_data: reader.read::<u32>()?,
			pointer_to_raw_data: reader.read::<u32>()?,
			pointer_to_relocations: reader.read::<u32>()?,
			pointer_to_line_numbers: reader.read::<u32>()?,
			number_of_relocations: reader.read::<u16>()?,
			number_of_line_numbers: reader.read::<u16>()?,
			characteristics: reader.read::<u32>()?,
		})
	}
}

impl<'data> PeInfo<'data> {
	pub fn parse(data: &'data [u8]) -> Result<Self> {
		if &data[0..2] != b"MZ" {
			return Err(PeError::NotPe);
		}

		let mut reader = BinaryReader::new(data);

		reader
			.seek(io::SeekFrom::Start(0x3c))
			.map_err(|_| PeError::InvalidHeader)?;

		let offset = reader.read::<u32>().map_err(|_| PeError::InvalidHeader)? as u64;
		reader
			.seek(io::SeekFrom::Start(offset))
			.map_err(|_| PeError::InvalidHeader)?;

		let mut signature = [0; 4];
		reader
			.read_slice::<u8>(&mut signature)
			.map_err(|_| PeError::InvalidHeader)?;

		if signature != *b"PE\0\0" {
			return Err(PeError::InvalidHeader);
		}

		let header = reader
			.read::<PeHeader>()
			.map_err(|_| PeError::InvalidHeader)?;
		let optional_header = reader
			.read::<PeOptionalHeader>()
			.map_err(|_| PeError::InvalidHeader)?;

		let data_directories = reader
			.read_array::<RvaAndSize>(match optional_header {
				PeOptionalHeader::Pe32(p) => p.number_of_data_directories,
				PeOptionalHeader::Pe64(p) => p.number_of_data_directories,
			} as usize)
			.map_err(|_| PeError::InvalidHeader)?;

		let sections = reader
			.read_array::<PeSectionHeader>(header.number_of_sections as usize)
			.map_err(|_| PeError::InvalidHeader)?;

		Ok(PeInfo {
			data,
			header,
			optional_header,
			data_directories,
			sections,
		})
	}

	pub fn resolve_rva(&self, rva: Rva) -> Result<usize> {
		let rva = rva.0;
		for section in self.sections.iter() {
			if rva >= section.virtual_address
				&& rva < section.virtual_address + section.size_of_raw_data
			{
				return Ok((rva - section.virtual_address + section.pointer_to_raw_data) as usize);
			}
		}
		Err(PeError::InvalidRva)
	}

	pub fn resolve_rva_slice(&self, range: RvaAndSize) -> Result<&'data [u8]> {
		let offset = self.resolve_rva(range.rva)?;
		Ok(&self.data[offset..offset + range.size as usize])
	}

	pub fn cli_header(&self) -> Option<CliHeader> {
		CliHeader::from_pe(self)
	}
}
