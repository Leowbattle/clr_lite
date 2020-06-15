#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! def_handle {
		($name:ident) => {
			#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
			pub struct $name(pub usize);

			impl Into<usize> for $name {
				fn into(self) -> usize {
					self.0
				}
			}

			impl Into<$name> for usize {
				fn into(self) -> $name {
					$name(self)
				}
			}
		};
	}
}

pub mod strings_heap;
pub use strings_heap::*;

pub mod user_strings_heap;
pub use user_strings_heap::*;

pub mod blob_heap;
pub use blob_heap::*;

pub mod blob;
pub use blob::*;

pub mod guid_heap;
pub use guid_heap::*;

#[macro_use]
pub mod tables_stream;
pub use tables_stream::*;

pub mod tables;

pub mod table_types;
pub use table_types::*;

pub mod token;
pub use token::*;

pub mod coded_index;
pub use coded_index::*;

/// ECMA-335 II.24.2.1
pub struct Root<'pe> {
	pub version: String,
	pub strings_heap: StringsHeap<'pe>,
	pub user_strings_heap: UserStringsHeap<'pe>,
	pub blob_heap: BlobHeap<'pe>,
	pub guid_heap: GuidHeap<'pe>,
	pub tables: TablesStream,
}

use std::fmt;

impl fmt::Debug for Root<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Root")
			.field("version", &self.version)
			.field("strings_heap", &self.strings_heap)
			.field("user_strings_heap", &self.user_strings_heap)
			.field("blob_heap", &self.blob_heap)
			.field("guid_heap", &self.guid_heap)
			.field("tables", &self.tables)
			.finish()
	}
}

use binary_reader::*;
use std::io::{self, Seek};

/// ECMA-335 II.24.2.2
struct StreamHeader<'pe> {
	name: String,
	data: &'pe [u8],
}

impl<'pe> Root<'pe> {
	pub(crate) fn from_data(data: &'pe [u8]) -> Option<Root<'pe>> {
		let mut reader = BinaryReader::new(data);

		if reader.read::<u32>().ok()? != 0x424A5342 {
			return None;
		}

		let _major_version = reader.read::<u16>().ok()?;
		let _minor_version = reader.read::<u16>().ok()?;
		let _reserved = reader.read::<u32>().ok()?;

		let version_length = reader.read::<u32>().ok()? as usize;

		let mut version = reader.read_string(version_length).ok()?;
		version.truncate(version.chars().position(|c| c == '\0')?);

		reader
			.seek(io::SeekFrom::Current(
				(version_length - version.len()) as i64,
			))
			.ok()?;

		let number_of_streams = reader.read::<u16>().ok()? as usize;
		let mut streams = Vec::with_capacity(number_of_streams);
		for _ in 0..number_of_streams {
			let offset = reader.read::<u32>().ok()? as usize;
			let size = reader.read::<u32>().ok()? as usize;
			let name = reader.read_null_terminated_string().ok()?;

			// Skip padding to multiple of 4
			reader
				.seek(io::SeekFrom::Current((name.len() as i64 + 4) & !3))
				.ok()?;

			streams.push(StreamHeader {
				name,
				data: &data[offset..offset + size],
			});
		}

		// ECMA-335 says "Streams need not be there if they are empty."
		// I will deal with this problem when I find as assembly that is missing a stream.

		let strings_heap =
			StringsHeap::new(streams.iter().find(|s| s.name == "#Strings").unwrap().data);

		let user_strings_heap =
			UserStringsHeap::new(streams.iter().find(|s| s.name == "#US").unwrap().data);

		let blob_heap = BlobHeap::new(streams.iter().find(|s| s.name == "#Blob").unwrap().data);

		let guid_heap = GuidHeap::new(streams.iter().find(|s| s.name == "#GUID").unwrap().data);

		let tables =
			TablesStream::new(streams.iter().find(|s| s.name == "#~").unwrap().data).unwrap();

		let root = Root {
			version,
			strings_heap,
			user_strings_heap,
			blob_heap,
			guid_heap,
			tables,
		};

		Some(root)
	}
}
