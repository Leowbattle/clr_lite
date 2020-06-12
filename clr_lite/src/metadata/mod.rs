#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! def_handle {
		($name:ident) => {
			pub struct $name(usize);

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

pub mod guid_heap;
pub use guid_heap::*;

pub mod tables_stream;
pub use tables_stream::*;

pub mod tables;

pub struct Root<'data> {
	data: &'data [u8],
	pub version: &'data str,
	pub strings_heap: Option<StringsHeap<'data>>,
	pub user_strings_heap: Option<UserStringsHeap<'data>>,
	pub blob_heap: Option<BlobHeap<'data>>,
	pub guid_heap: Option<GuidHeap<'data>>,
	pub tables: Option<TablesStream<'data>>,
}
