#![allow(non_upper_case_globals)]

pub mod table_type;
pub use table_type::*;

pub mod coded_index;
pub use coded_index::*;

pub mod module;
pub use module::*;

pub mod type_ref;
pub use type_ref::*;

pub mod type_def;
pub use type_def::*;

pub mod field;
pub use field::*;

pub mod method_def;
pub use method_def::*;

pub mod param;
pub use param::*;

pub mod interface_impl;
pub use interface_impl::*;

pub mod member_ref;
pub use member_ref::*;

pub mod constant;
pub use constant::*;

pub mod custom_attribute;
pub use custom_attribute::*;

pub mod field_marshal;
pub use field_marshal::*;

pub mod decl_security;
pub use decl_security::*;

pub mod class_layout;
pub use class_layout::*;

pub mod field_layout;
pub use field_layout::*;

pub mod standalone_sig;
pub use standalone_sig::*;

pub mod event_map;
pub use event_map::*;

pub mod event;
pub use event::*;

pub mod property_map;
pub use property_map::*;

pub mod property;
pub use property::*;

pub mod method_semantics;
pub use method_semantics::*;

pub mod method_impl;
pub use method_impl::*;

pub mod module_ref;
pub use module_ref::*;

pub mod type_spec;
pub use type_spec::*;

pub mod impl_map;
pub use impl_map::*;

pub mod field_rva;
pub use field_rva::*;

pub mod assembly;
pub use assembly::*;

pub mod assembly_processor;
pub use assembly_processor::*;

pub mod assembly_os;
pub use assembly_os::*;

pub mod assembly_ref;
pub use assembly_ref::*;

pub mod assembly_ref_processor;
pub use assembly_ref_processor::*;

pub mod assembly_ref_os;
pub use assembly_ref_os::*;

pub mod file;
pub use file::*;

pub mod exported_type;
pub use exported_type::*;

pub mod manifest_resource;
pub use manifest_resource::*;

pub mod nested_class;
pub use nested_class::*;

pub mod generic_param;
pub use generic_param::*;

pub mod method_spec;
pub use method_spec::*;

pub mod generic_param_constraint;
pub use generic_param_constraint::*;

use crate::metadata::*;

#[derive(Debug)]
pub struct Table<T: TableRow>(Box<[T]>);

impl<T: TableRow> Table<T> {
	pub fn rows(&self) -> &[T] {
		&self.0
	}
}

impl<T: TableRow> std::ops::Index<T::Handle> for Table<T> {
	type Output = T;

	fn index(&self, h: T::Handle) -> &Self::Output {
		&self.0[h.into()]
	}
}

pub struct Tables {
	pub module: Table<Module>,
	pub type_ref: Table<TypeRef>,
	pub type_def: Table<TypeDef>,
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
	wide_table_handles: [bool; 64],
	wide_string: bool,
	wide_guid: bool,
	wide_blob: bool,

	wide_type_def_or_ref: bool,
	wide_resolution_scope: bool,
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

		let mut wide_table_handles = [false; 64];
		for i in 0..64 {
			wide_table_handles[i] = row_counts[i] > 65535;
		}

		TableReader {
			reader: br,
			row_counts,

			wide_table_handles,
			wide_string: heap_sizes.contains(HeapSizeFlags::String),
			wide_guid: heap_sizes.contains(HeapSizeFlags::Guid),
			wide_blob: heap_sizes.contains(HeapSizeFlags::Blob),

			wide_type_def_or_ref: is_coded_index_wide(
				TypeDefOrRefHandle::LARGE_ROW_SIZE,
				TypeDefOrRefHandle::TABLES,
				&row_counts,
			),
			wide_resolution_scope: is_coded_index_wide(
				ResolutionScopeHandle::LARGE_ROW_SIZE,
				ResolutionScopeHandle::TABLES,
				&row_counts,
			),
		}
		.read_tables()
	}

	fn read_tables(mut self) -> Result<Tables, TableReaderError> {
		macro_rules! get_table {
			($type:ident) => {{
				let count = self.row_counts[TableType::$type as usize];
				let mut table = Vec::with_capacity(count);

				// TODO: Investigate why assert_eq!(table.capacity(), count) fails
				// assert_eq!(table.capacity(), count);

				for _ in 0..count {
					table.push($type::read_row(&mut self)?);
					}
				Table::<$type>(table.into_boxed_slice())
				}};
		}

		Ok(Tables {
			module: get_table!(Module),
			type_ref: get_table!(TypeRef),
			type_def: get_table!(TypeDef),
		})
	}

	fn _read<T: CopyFromBytes>(&mut self) -> Result<T, TableReaderError> {
		self.reader
			.read::<T>()
			.ok_or_else(|| TableReaderError::BadImageFormat("Unexpected EOF".to_string()))
	}

	pub fn read_field_handle(&mut self) -> Result<FieldHandle, TableReaderError> {
		match self.wide_table_handles[TableType::Field as usize] {
			true => Ok(FieldHandle(self._read::<u32>()? as usize)),
			false => Ok(FieldHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_method_def_handle(&mut self) -> Result<MethodDefHandle, TableReaderError> {
		match self.wide_table_handles[TableType::MethodDef as usize] {
			true => Ok(MethodDefHandle(self._read::<u32>()? as usize)),
			false => Ok(MethodDefHandle(self._read::<u16>()? as usize)),
		}
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

	pub fn read_type_def_or_ref_handle(&mut self) -> Result<TypeDefOrRefHandle, TableReaderError> {
		let data = match self.wide_type_def_or_ref {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & TypeDefOrRefHandle::TAG_MASK;
		let index = (data & !TypeDefOrRefHandle::TAG_MASK) >> (TypeDefOrRefHandle::TAG_MASK);

		Ok(match tag {
			0 => TypeDefOrRefHandle::TypeDefHandle(TypeDefHandle(index)),
			1 => TypeDefOrRefHandle::TypeRefHandle(TypeRefHandle(index)),
			2 => TypeDefOrRefHandle::TypeSpecHandle(TypeSpecHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid TypeDefOrRef tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_resolution_scope(&mut self) -> Result<ResolutionScopeHandle, TableReaderError> {
		let data = match self.wide_resolution_scope {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & ResolutionScopeHandle::TAG_MASK;
		let index = (data & !ResolutionScopeHandle::TAG_MASK) >> (ResolutionScopeHandle::TAG_MASK);

		Ok(match tag {
			0 => ResolutionScopeHandle::ModuleHandle(ModuleHandle(index)),
			1 => ResolutionScopeHandle::ModuleRefHandle(ModuleRefHandle(index)),
			2 => ResolutionScopeHandle::AssemblyRefHandle(AssemblyRefHandle(index)),
			3 => ResolutionScopeHandle::TypeRefHandle(TypeRefHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid ResolutionScope tag {}",
					tag
				)))
			}
		})
	}
}

fn is_coded_index_wide(large_row_size: usize, tables: &[TableType], row_counts: &[usize]) -> bool {
	tables
		.iter()
		.any(|&t| row_counts[t as usize] > large_row_size)
}
