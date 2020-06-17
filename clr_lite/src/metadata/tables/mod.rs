#![allow(non_upper_case_globals)]

pub mod table_type;
pub use table_type::*;

pub mod coded_index;
pub use coded_index::*;

pub mod module;
pub use module::*;

use crate::metadata::*;

#[derive(Debug)]
pub struct Table<T: TableRow>(Box<[T]>);

impl<T: TableRow> std::ops::Index<T::Handle> for Table<T> {
	type Output = T;

	fn index(&self, h: T::Handle) -> &Self::Output {
		&self.0[h.into()]
	}
}

pub struct Tables {
	pub module: Table<Module>,
}

pub trait TableRow: Sized + std::fmt::Debug {
	type Handle: Into<usize>;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Self, TableReaderError>;
}

use binary_reader::*;

#[derive(Debug)]
pub enum TableReaderError {
	BadImageFormat(String),
}

impl ToString for TableReaderError {
	fn to_string(&self) -> String {
		match &self {
			TableReaderError::BadImageFormat(s) => s.clone(),
		}
	}
}

pub struct TableReader<'data> {
	reader: BinaryReader<'data>,
	row_counts: [usize; 64],

	// The following variables say if an index is encoded with 2 or 4 bytes
	wide_string: bool,
	wide_guid: bool,
	wide_blob: bool,
}

bitflags! {
	struct HeapSizeFlags : u8 {
		const String = 1;
		const Guid = 2;
		const Blob = 4;
	}
}

impl<'data> TableReader<'data> {
	pub(crate) fn read(data: &'data [u8]) -> Result<Tables, TableReaderError> {
		// Read tables header
		// ECMA II.24.2.6

		let mut br = BinaryReader::new(data);

		// Skip unused data
		br.advance(6);

		let heap_sizes = br
			.read::<u8>()
			.and_then(|bits| HeapSizeFlags::from_bits(bits))
			.ok_or_else(|| {
				TableReaderError::BadImageFormat("Invalid metadata header".to_string())
			})?;

		// Skip unused data
		br.advance(1);

		let present_tables = br.read::<u64>().ok_or_else(|| {
			TableReaderError::BadImageFormat("Invalid metadata header".to_string())
		})?;
		let sorted_tables = br.read::<u64>().ok_or_else(|| {
			TableReaderError::BadImageFormat("Invalid metadata header".to_string())
		})?;

		let row_counts_raw = br
			.read_array::<u32>(present_tables.count_ones() as usize)
			.ok_or_else(|| {
				TableReaderError::BadImageFormat("Invalid metadata header".to_string())
			})?;

		let mut used = 0;
		let mut row_counts = [0; 64];
		for i in 0..64 {
			if present_tables & (1 << i) != 0 {
				row_counts[i] = row_counts_raw[used] as usize;
				used += 1;
			}
		}

		TableReader {
			reader: br,
			row_counts,

			wide_string: heap_sizes.contains(HeapSizeFlags::String),
			wide_guid: heap_sizes.contains(HeapSizeFlags::Guid),
			wide_blob: heap_sizes.contains(HeapSizeFlags::Blob),
		}
		.read_tables()
	}

	fn read_tables(mut self) -> Result<Tables, TableReaderError> {
		macro_rules! get_table {
			($type:ident) => {{
				let mut table = Vec::with_capacity(self.row_counts[TableType::$type as usize]);
				for _ in 0..table.capacity() {
					table.push($type::read_row(&mut self)?);
					}
				Table::<$type>(table.into_boxed_slice())
				}};
		}

		Ok(Tables {
			module: get_table!(Module),
		})
	}

	fn _read<T: CopyFromBytes>(&mut self) -> Result<T, TableReaderError> {
		self.reader
			.read::<T>()
			.ok_or_else(|| TableReaderError::BadImageFormat("Unexpected EOF".to_string()))
	}

	pub fn read_string_handle(&mut self) -> Result<StringHandle, TableReaderError> {
		match self.wide_string {
			true => Ok(StringHandle(self._read::<u32>()? as usize)),
			false => Ok(StringHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_guid_handle(&mut self) -> Result<GuidHandle, TableReaderError> {
		match self.wide_guid {
			true => Ok(GuidHandle(self._read::<u32>()? as usize)),
			false => Ok(GuidHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_blob_handle(&mut self) -> Result<BlobHandle, TableReaderError> {
		match self.wide_string {
			true => Ok(BlobHandle(self._read::<u32>()? as usize)),
			false => Ok(BlobHandle(self._read::<u16>()? as usize)),
		}
	}
}

fn is_coded_index_wide(large_row_size: usize, row_counts: &[usize], tables: &[TableType]) -> bool {
	tables
		.iter()
		.any(|&t| row_counts[t as usize] > large_row_size)
}
